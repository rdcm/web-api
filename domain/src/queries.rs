use async_trait::async_trait;
use bson::serde_helpers::{
    deserialize_hex_string_from_object_id, serialize_hex_string_as_object_id,
};
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
    #[serde(
        rename = "_id",
        deserialize_with = "deserialize_hex_string_from_object_id",
        serialize_with = "serialize_hex_string_as_object_id",
        skip_serializing_if = "String::is_empty"
    )]
    pub id: String,
    pub name: String,
    pub age: u8,
}
