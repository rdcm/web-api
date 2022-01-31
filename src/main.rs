mod contracts;
mod user;
mod dal;

use user::user_module::*;
use contracts::contracts_module::*;
use dal::dal_module::*;

use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{Client, Collection};

#[macro_use] extern crate serde_derive;

#[get("/user/{user_id}")]
async fn get(
    user_repo: web::Data<UserRepository>,
    user_id: web::Path<String>) -> HttpResponse {


    let option = user_repo.get(user_id.to_string()).await;

    match option {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound()
            .body(format!("No user found with id {}", user_id)),   
    }
    
}

#[post("/user")]
async fn create(
    user_repo: web::Data<UserRepository>,
    user: Json<CreateUserRequest>) -> HttpResponse {
        let user = User {
            name: user.name.to_string(),
            age: user.age,
        };
    
        let result = user_repo.create(user).await;
    
        match result {
            Ok(_) => HttpResponse::Ok().body("user added"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed connect to db");
    let collection: Collection<User> = client.database("rust_app").collection("users");

    let user_repo  = UserRepository::new(collection);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .service(get)
            .service(create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
