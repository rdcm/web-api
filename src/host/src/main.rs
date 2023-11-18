use domain::events::ActivityEvent;
use host::composition::Composition;
use host::conf::AppConf;
use messaging::kafka::{IKafkaFactory, KafkaConfig, KafkaFactory};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = KafkaConfig::new();
    let kafka_factory: Arc<dyn IKafkaFactory<ActivityEvent>> = Arc::new(KafkaFactory::new(config));

    let conf = AppConf::new();
    Composition::new(&conf, kafka_factory).await?.server.await
}
