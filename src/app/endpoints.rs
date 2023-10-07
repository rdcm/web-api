use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;

use crate::app::models::CreateUserRequest;
use crate::contracts::commands::{CreateUserCommand, ICommandHandler};
use crate::contracts::queries::{GetUserQuery, IQueryHandler, User};

pub async fn get_user(
    handler: Data<dyn IQueryHandler<GetUserQuery, Option<User>>>,
    path: Path<String>,
) -> HttpResponse {
    let query = GetUserQuery {
        id: path.to_string(),
    };

    let option = handler.handle(query).await;

    match option {
        Some(user) => HttpResponse::Ok().json(user),
        None => {
            HttpResponse::NotFound().body(format!("No user found with id {}", path))
        }
    }
}

pub async fn create_user(
    handler: Data<dyn ICommandHandler<CreateUserCommand, Option<String>>>,
    request: Json<CreateUserRequest>,
) -> HttpResponse {
    let command = CreateUserCommand {
        name: request.name.to_string(),
        age: request.age,
    };

    let option = handler.handle(command).await;

    match option {
        Some(id) => HttpResponse::Ok().body(id),
        None => HttpResponse::BadRequest().body(()),
    }
}
