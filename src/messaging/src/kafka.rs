use async_trait::async_trait;
use log::{error, info, warn};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

#[derive(Clone)]
pub struct KafkaConfig {
    brokers: String,
    topic: String,
    group_id: String,
}

impl KafkaConfig {
    pub fn new() -> Self {
        Self {
            brokers: std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into()),
            topic: std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "messages".into()),
            group_id: std::env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "local".into()),
        }
    }
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IKafkaFactory<T: for<'a> Deserialize<'a> + Serialize + Send + Sync + Debug> {
    fn create_producer(&self) -> Arc<dyn IKafkaProducer<T>>;
    fn create_consumer(&self) -> Arc<dyn IKafkaConsumer<T>>;
}

#[async_trait]
pub trait IKafkaProducer<T>: Sync + Send
where
    T: Serialize + Send + Sync,
{
    async fn produce(&self, key: &str, message: &T) -> Option<()>;
}

#[async_trait]
pub trait IKafkaConsumer<T: for<'a> Deserialize<'a> + Debug> {
    async fn consume(&self, handler: &(dyn Fn(T) + Sync));
    fn poll(&self) -> T;
}

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(config: KafkaConfig) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", config.brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        let topic = config.topic;

        Self { producer, topic }
    }
}

#[async_trait]
impl<T: Serialize + Send + Sync> IKafkaProducer<T> for KafkaProducer {
    async fn produce(&self, key: &str, message: &T) -> Option<()> {
        let json = serde_json::to_string(&message).ok()?;
        let record = FutureRecord::to(&self.topic).key(key).payload(&json);

        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .ok()?;

        Some(())
    }
}

pub struct KafkaConsumer {
    consumer: StreamConsumer<CustomContext>,
}

impl KafkaConsumer {
    pub fn new(config: KafkaConfig) -> KafkaConsumer {
        let context = CustomContext;

        let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
            .set("group.id", config.group_id)
            .set("bootstrap.servers", config.brokers)
            .set("auto.offset.reset", "earliest")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&config.topic])
            .expect("Can't subscribe to specified topics");

        Self { consumer }
    }
}

#[async_trait]
impl<T: for<'a> Deserialize<'a> + Debug> IKafkaConsumer<T> for KafkaConsumer {
    async fn consume(&self, handler: &(dyn Fn(T) + Sync)) {
        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    match m.payload() {
                        None => error!("empty payload"),
                        Some(p) => match serde_json::from_slice::<T>(p) {
                            Err(e) => warn!("Serialization error: {}", e),
                            Ok(message) => handler(message),
                        },
                    };
                }
            };
        }
    }

    fn poll(&self) -> T {
        todo!()
    }
}

pub struct KafkaFactory {
    config: KafkaConfig,
}

impl KafkaFactory {
    pub fn new(config: KafkaConfig) -> KafkaFactory {
        Self { config }
    }
}

impl<T: for<'a> Deserialize<'a> + Serialize + Send + Sync + Debug> IKafkaFactory<T>
    for KafkaFactory
{
    fn create_producer(&self) -> Arc<dyn IKafkaProducer<T>> {
        let producer = KafkaProducer::new(self.config.clone());
        Arc::new(producer)
    }

    fn create_consumer(&self) -> Arc<dyn IKafkaConsumer<T>> {
        let consumer = KafkaConsumer::new(self.config.clone());
        Arc::new(consumer)
    }
}
