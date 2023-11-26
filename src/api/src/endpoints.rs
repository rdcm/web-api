use crate::models::TrackActivityRequest::{Click, Open};
use crate::models::{
    CreateUserRequest, CreatedUserIdResponse, ErrorResponse, TrackActivityRequest, UserResponse,
};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse};
use domain::commands::{CreateUserCommand, ICommandHandler};
use domain::events::{ActivityEvent, IActivityTracker};
use domain::queries::{GetUserQuery, IQueryHandler, User};

/// Get user
///
/// Get user by id
#[utoipa::path(
    get,
    path = "/user/{user_id}",
    responses(
        (status = 200, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Something going wrong", body = ErrorResponse)
    )
)]
#[get("/user/{user_id}")]
pub async fn get_user(
    handler: Data<dyn IQueryHandler<GetUserQuery, Option<User>>>,
    path: Path<String>,
) -> HttpResponse {
    let query = GetUserQuery {
        id: path.into_inner(),
    };

    let option = handler.handle(query).await;

    match option {
        Some(user) => HttpResponse::Ok().json(UserResponse {
            name: user.name,
            age: user.age,
            id: user.id,
        }),
        None => HttpResponse::BadRequest().json(ErrorResponse { code: 101 }),
    }
}

/// Create user
///
/// Create user
#[utoipa::path(
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = CreatedUserIdResponse),
        (status = 400, description = "Something going wrong", body = ErrorResponse)
    )
)]
#[post("/user")]
pub async fn create_user(
    handler: Data<dyn ICommandHandler<CreateUserCommand, Option<String>>>,
    json: Json<CreateUserRequest>,
) -> HttpResponse {
    let request = json.into_inner();

    let command = CreateUserCommand {
        name: request.name,
        age: request.age,
    };

    let option = handler.handle(command).await;

    match option {
        Some(id) => HttpResponse::Created().json(CreatedUserIdResponse { id }),
        None => HttpResponse::BadRequest().json(ErrorResponse { code: 102 }),
    }
}

/// Track user activity
///
/// Track user activity
#[utoipa::path(
    request_body = TrackActivityRequest,
    responses(
        (status = 200, description = "Activity successfully tracked"),
    )
)]
#[post("/track_activity")]
pub async fn track_activity(
    tracker: Data<dyn IActivityTracker>,
    json: Json<TrackActivityRequest>,
) -> HttpResponse {
    let request = json.into_inner();

    let result = match request {
        Click { x, y } => tracker.track(&ActivityEvent::Click { x, y }).await,
        Open { path } => tracker.track(&ActivityEvent::Open { p: path }).await,
    };

    match result {
        Some(_) => HttpResponse::Ok().json(()),
        None => HttpResponse::BadRequest().json(ErrorResponse { code: 103 })
    }
}
