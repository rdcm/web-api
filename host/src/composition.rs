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
use app::endpoints::{create_user, get_user};
use app::models::*;
use domain::commands::{CreateUserCommand, ICommandHandler};
use domain::queries::{GetUserQuery, IQueryHandler, User};
use domain_impl::handlers::{CreateUserCommandHandler, GetUserQueryHandler};
use infra::repositories::UserRepository;

#[derive(OpenApi)]
#[openapi(
    paths(app::endpoints::create_user, app::endpoints::get_user),
    components(
        schemas(CreateUserRequest, CreatedUserIdResponse, ErrorResponse),
        schemas(UserResponse, ErrorResponse)
    )
)]
struct ApiDoc;
pub struct Composition {
    pub server: Server,
    pub addrs: Vec<SocketAddr>,
}

impl Composition {
    pub async fn new(conf: &AppConf) -> Result<Composition, std::io::Error> {
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
                .app_data(Data::from(command_handler.clone()))
                .app_data(Data::from(query_handler.clone()))
        })
        .bind(&addr)
        .unwrap_or_else(|_| panic!("Failed to bind to the host: {}", &addr));

        let addrs = http_server.addrs();
        let server = http_server.run();

        Ok(Self { server, addrs })
    }
}
