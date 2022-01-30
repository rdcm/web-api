pub mod user_module {
    use serde::{Deserialize, Serialize};

    use async_trait::async_trait;

    #[async_trait]
    pub trait IUserRepository {
        async fn create(&self, user: User) -> Result<String, String>;
        async fn get(&self, id: String) -> Option<User>;
    }

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    pub struct User {
        pub name: String,
        pub age: u8,
    }
}
