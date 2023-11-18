use async_trait::async_trait;
use domain::events::{ActivityEvent, IActivityTracker};
use messaging::kafka::IKafkaProducer;
use std::sync::Arc;

#[derive(Clone)]
pub struct ActivityTracker {
    producer: Arc<dyn IKafkaProducer<ActivityEvent>>,
}

impl ActivityTracker {
    pub fn new(producer: Arc<dyn IKafkaProducer<ActivityEvent>>) -> ActivityTracker {
        ActivityTracker { producer }
    }
}

#[async_trait]
impl IActivityTracker for ActivityTracker {
    async fn track(&self, activity: &ActivityEvent) {
        self.producer.produce(activity).await;
    }
}
