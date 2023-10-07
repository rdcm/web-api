mod app;
mod contracts;
mod domain;
mod infra;
mod root;

use crate::root::composition_root::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    create_server(8080).await.unwrap().server.await
}
