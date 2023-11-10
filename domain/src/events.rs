use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

#[async_trait]
pub trait IActivityTracker: Send + Sync {
    async fn track(&self, activity: &ActivityEvent);
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityEvent {
    #[serde(rename = "click")]
    Click { x: i32, y: i32 },
    #[serde(rename = "open")]
    Open { p: String },
}
