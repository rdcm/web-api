use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Client, Collection};

use domain::queries::User;
use domain_impl::ports::IUserRepository;

#[derive(Debug, Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(client: Client, db_name: &str) -> Self {
        let collection: Collection<User> = client.database(db_name).collection("users");
        Self { collection }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: User) -> Option<String> {
        let user_id = self
            .collection
            .insert_one(user, None)
            .await
            .ok()?
            .inserted_id
            .as_object_id()?
            .to_string();

        Some(user_id)
    }

    async fn get(&self, id: String) -> Option<User> {
        let user_id = ObjectId::parse_str(&id).ok()?;

        self.collection
            .find_one(doc! { "_id": user_id }, None)
            .await
            .ok()?
    }
}
