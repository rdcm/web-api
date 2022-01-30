mod user;
mod contracts;

use user::user_module::{User};
use contracts::contracts_module::{UserRequest};

use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, http};

#[macro_use] extern crate serde_derive;

#[get("/user/get")]
async fn get() -> impl Responder {
    let user = User {
        name: String::from("someusername123"),
        age: 20,
        id: 1,
    };

    HttpResponse::Ok().body(user.name)
}

#[post("/user/create")]
async fn create(user: Json<UserRequest>) -> impl Responder {
    HttpResponse::Ok().body(&user.name)
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
