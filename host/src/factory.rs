use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::{Client, Collection};
use std::net::SocketAddr;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

pub struct ServerInfo {
    pub server: Server,
    pub addrs: Vec<SocketAddr>,
}

pub async fn create_server(port: i32) -> Result<ServerInfo, std::io::Error> {
    let user_repository_arc = Arc::new(create_user_repository().await);
    let command_handler: Arc<dyn ICommandHandler<CreateUserCommand, Option<String>>> =
        Arc::new(CreateUserCommandHandler {
            repo: user_repository_arc.clone(),
        });
    let query_handler: Arc<dyn IQueryHandler<GetUserQuery, Option<User>>> =
        Arc::new(GetUserQueryHandler {
            repo: user_repository_arc.clone(),
        });

    let openapi = ApiDoc::openapi();

    let http_server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(create_user)
            .service(get_user)
            .app_data(Data::from(command_handler.clone()))
            .app_data(Data::from(query_handler.clone()))
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect("Failed to bind to the server.");

    let addrs = http_server.addrs();
    let server = http_server.run();

    Ok(ServerInfo { server, addrs })
}

async fn create_user_repository() -> UserRepository {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let db_name = std::env::var("MONGODB_DBNAME").unwrap_or_else(|_| "rust_app".into());

    let client = Client::with_uri_str(uri)
        .await
        .expect("failed connect to db");
    let collection: Collection<User> = client.database(&db_name).collection("users");
    UserRepository::new(collection)
}
