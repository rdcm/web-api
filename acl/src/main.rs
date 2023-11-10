use domain::events::ActivityEvent;
use log::info;
use messaging::kafka::{IKafkaFactory, KafkaConfig, KafkaFactory};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = KafkaConfig::new();
    let kafka_factory: Arc<dyn IKafkaFactory<ActivityEvent>> = Arc::new(KafkaFactory::new(config));
    kafka_factory
        .create_consumer()
        .consume(&|message| {
            info!("message: '{:?}'", message);
        })
        .await;
}
