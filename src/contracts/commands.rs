use async_trait::async_trait;

#[async_trait]
pub trait ICommandHandler<T, Option>: Send + Sync {
    async fn handle(&self, cmd: T) -> Option;
}

pub struct CreateUserCommand {
    pub name: String,
    pub age: u8,
}