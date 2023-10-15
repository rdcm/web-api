use host::conf::read_app_conf;
use host::factory::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conf = read_app_conf();
    create_server(&conf).await.unwrap().server.await
}
