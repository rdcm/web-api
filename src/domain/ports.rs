use async_trait::async_trait;
use crate::contracts::queries::User;

#[async_trait]
pub trait IUserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> Option<String>;
    async fn get(&self, id: String) -> Option<User>;
}