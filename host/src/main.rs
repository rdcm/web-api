use host::composition::Composition;
use host::conf::AppConf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conf = AppConf::new();
    Composition::new(&conf).await?.server.await
}
