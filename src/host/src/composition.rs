use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::conf::AppConf;
use api::endpoints::{create_user, get_user, track_activity};
use api::models::*;
use domain::commands::{CreateUserCommand, ICommandHandler};
use domain::events::{ActivityEvent, IActivityTracker};
use domain::queries::{GetUserQuery, IQueryHandler, User};
use domain_impl::handlers::{CreateUserCommandHandler, GetUserQueryHandler};
use infra::repositories::UserRepository;
use infra::tracker::ActivityTracker;
use messaging::kafka::IKafkaFactory;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::endpoints::create_user,
        api::endpoints::get_user,
        api::endpoints::track_activity
    ),
    components(
        schemas(CreateUserRequest, CreatedUserIdResponse, ErrorResponse),
        schemas(UserResponse, ErrorResponse),
        schemas(TrackActivityRequest)
    )
)]
struct ApiDoc;
pub struct Composition {
    pub server: Server,
    pub addrs: Vec<SocketAddr>,
}

impl Composition {
    pub async fn new(
        conf: &AppConf,
        kafka_factory: Arc<dyn IKafkaFactory<ActivityEvent>>,
    ) -> Result<Composition, std::io::Error> {
        let producer = kafka_factory.create_producer();
        let tracker: Arc<dyn IActivityTracker> = Arc::new(ActivityTracker::new(producer));

        let mut client_options = ClientOptions::parse(&conf.db_uri).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();

        let user_repository = UserRepository::new(client, &conf.db_name);
        let user_repository_arc = Arc::new(user_repository);

        let command_handler: Arc<dyn ICommandHandler<CreateUserCommand, Option<String>>> =
            Arc::new(CreateUserCommandHandler {
                repo: user_repository_arc.clone(),
            });

        let query_handler: Arc<dyn IQueryHandler<GetUserQuery, Option<User>>> =
            Arc::new(GetUserQueryHandler {
                repo: user_repository_arc.clone(),
            });

        let openapi = ApiDoc::openapi();

        let addr = conf.get_api_address();
        let http_server = HttpServer::new(move || {
            App::new()
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", openapi.clone()),
                )
                .service(create_user)
                .service(get_user)
                .service(track_activity)
                .app_data(Data::from(command_handler.clone()))
                .app_data(Data::from(query_handler.clone()))
                .app_data(Data::from(tracker.clone()))
        })
        .bind(&addr)
        .unwrap_or_else(|_| panic!("Failed to bind to the host: {}", &addr));

        let addrs = http_server.addrs();
        let server = http_server.run();

        Ok(Self { server, addrs })
    }
}
