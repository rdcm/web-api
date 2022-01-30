pub mod dal_module {
    use crate::{IUserRepository, User};
    use mongodb::{bson, bson::doc, Collection };
    use async_trait::async_trait;

    #[derive(Debug, Clone)]
    pub struct UserRepository {
        pub coll: Collection<User>,
    }

    #[async_trait]
    impl IUserRepository for UserRepository {
        async fn create(&self, user: User) -> Result<String, String> {
            let result = self.coll.insert_one(user, None).await;
            return match result {
                Ok(insertion_result) => Result::Ok(insertion_result.inserted_id.to_string()),
                Err(err) => Result::Err(err.to_string())
            };
        }

        async fn get(&self, id: String) -> Option<User> {
            let result = self.coll.find_one(doc! { "_id": bson::oid::ObjectId::parse_str(&id).unwrap() }, None).await;

            return match result {
                Ok(find_result) => find_result,
                Err(err) => Option::None
            };
        }
    }
}