mod contracts;

use contracts::contracts_module::{UserResponse};
use contracts::contracts_module::{CreateUserRequest};
use contracts::contracts_module::{UserIdResponse};

use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, http};
use crate::http::StatusCode;

#[macro_use] extern crate serde_derive;

#[get("/user/get")]
async fn get() -> Result<Json<UserResponse>> {

    let user_id = 1;
    let response = UserResponse {
        name: String::from("someusername123"),
        age: 20,
        id: user_id,
    };

    Ok(Json(response))
}

#[post("/user/create")]
async fn create(user: Json<CreateUserRequest>) -> Result<Json<UserIdResponse>> {

    let response = UserIdResponse {
        id: 1
    };

    Ok(Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
