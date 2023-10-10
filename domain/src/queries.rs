use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

#[async_trait]
pub trait IQueryHandler<T, V>: Send + Sync {
    async fn handle(&self, query: T) -> V;
}

pub struct GetUserQuery {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}
