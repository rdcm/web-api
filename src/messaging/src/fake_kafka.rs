use crate::kafka::{IKafkaConsumer, IKafkaFactory, IKafkaProducer};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

pub struct FakeKafkaFactory {
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Receiver<String>>,
}

impl Default for FakeKafkaFactory {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FakeKafkaProducer {
    sender: Arc<SyncSender<String>>,
}
pub struct FakeKafkaConsumer {
    receiver: Arc<Receiver<String>>,
}

unsafe impl Send for FakeKafkaConsumer {}
unsafe impl Sync for FakeKafkaConsumer {}

impl<T: for<'a> Deserialize<'a> + Serialize + Send + Sync + Debug> IKafkaFactory<T>
    for FakeKafkaFactory
{
    fn create_producer(&self) -> Arc<dyn IKafkaProducer<T>> {
        let producer = FakeKafkaProducer::new(self.sender.clone());
        Arc::new(producer)
    }

    fn create_consumer(&self) -> Arc<dyn IKafkaConsumer<T>> {
        let consumer = FakeKafkaConsumer::new(self.receiver.clone());
        Arc::new(consumer)
    }
}

#[async_trait]
impl<T: Serialize + Send + Sync> IKafkaProducer<T> for FakeKafkaProducer {
    async fn produce(&self, _key: &str, message: &T) -> Option<()> {
        let json = serde_json::to_string(message).ok()?;
        self.sender.send(json).ok()?;
        Some(())
    }
}

#[async_trait]
impl<T: for<'a> Deserialize<'a> + Debug> IKafkaConsumer<T> for FakeKafkaConsumer {
    async fn consume(&self, _handler: &(dyn Fn(T) + Sync)) {
        unimplemented!();
    }

    fn poll(&self) -> T {
        let message = self.receiver.recv().unwrap();
        serde_json::from_str::<T>(&message).unwrap()
    }
}

impl FakeKafkaFactory {
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        let (sender, receiver): (SyncSender<String>, Receiver<String>) = sync_channel(10);
        let sender_arc = Arc::new(sender);
        let receiver_arc = Arc::new(receiver);

        Self {
            sender: sender_arc,
            receiver: receiver_arc,
        }
    }
}

impl FakeKafkaProducer {
    pub fn new(sender: Arc<SyncSender<String>>) -> Self {
        Self { sender }
    }
}

impl FakeKafkaConsumer {
    pub fn new(receiver: Arc<Receiver<String>>) -> Self {
        Self { receiver }
    }
}
