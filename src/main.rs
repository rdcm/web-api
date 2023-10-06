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
    return create_server().await.unwrap().await;
}
