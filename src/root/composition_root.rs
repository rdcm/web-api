use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use actix_web::web::{Data, get, post};
use mongodb::{Client, Collection};

use crate::app::endpoints::{create_user, get_user};
use crate::contracts::commands::{CreateUserCommand, ICommandHandler};
use crate::contracts::queries::{GetUserQuery, IQueryHandler, User};
use crate::domain::handlers::{CreateUserCommandHandler, GetUserQueryHandler};
use crate::infra::repositories::UserRepository;

pub async fn create_server() -> Result<Server, std::io::Error> {

    let user_repository = create_user_repository().await;
    let command_handler: Arc<dyn ICommandHandler<CreateUserCommand, Option<String>>> = Arc::new(CreateUserCommandHandler {
        repo: Arc::new(user_repository.clone()),
    });
    let query_handler: Arc<dyn IQueryHandler<GetUserQuery, Option<User>>> = Arc::new(GetUserQueryHandler {
        repo: Arc::new(user_repository.clone()),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::from(command_handler.clone()))
            .app_data(Data::from(query_handler.clone()))
            .route("/user/{user_id}", get().to(get_user))
            .route("/user", post().to(create_user))

    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind to the server.")
    .run();

    Ok(server)
}

async fn create_user_repository() -> UserRepository {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let db_name = std::env::var("MONGODB_DBNAME")
        .unwrap_or_else(|_| "rust_app".into());

    let client = Client::with_uri_str(uri).await.expect("failed connect to db");
    let collection: Collection<User> = client.database(&db_name).collection("users");
    return UserRepository::new(collection);
}