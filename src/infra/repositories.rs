use mongodb::{bson, bson::doc, Collection };
use async_trait::async_trait;

use crate::contracts::queries::User;
use crate::domain::ports::IUserRepository;

#[derive(Debug, Clone)]
pub struct UserRepository {
    collection: Collection<User>
}

impl UserRepository {
    pub fn new(collection: Collection<User>) -> UserRepository {
        UserRepository {
            collection,
        }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: User) -> Option<String> {
        let result = self.collection.insert_one(user, None).await;
        return match result {
            Ok(insertion_result) => Some(insertion_result.inserted_id.to_string()),
            Err(_) => None
        };
    }

    async fn get(&self, id: String) -> Option<User> {
        let user_id = bson::oid::ObjectId::parse_str(&id).ok()?;
        return self.collection.find_one(doc! { "_id": user_id }, None).await.ok()?;
    }
}