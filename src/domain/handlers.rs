use crate::contracts::commands::{CreateUserCommand, ICommandHandler};
use crate::contracts::queries::{GetUserQuery, IQueryHandler, User};
use crate::domain::ports::IUserRepository;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateUserCommandHandler {
    pub repo: Arc<dyn IUserRepository>,
}

#[async_trait]
impl ICommandHandler<CreateUserCommand, Option<String>> for CreateUserCommandHandler {
    async fn handle(&self, cmd: CreateUserCommand) -> Option<String> {
        let user = User {
            name: cmd.name.to_string(),
            age: cmd.age,
        };

        self.repo.create(user).await
    }
}

pub struct GetUserQueryHandler {
    pub repo: Arc<dyn IUserRepository>,
}

#[async_trait]
impl IQueryHandler<GetUserQuery, Option<User>> for GetUserQueryHandler {
    async fn handle(&self, query: GetUserQuery) -> Option<User> {
        self.repo.get(query.id).await
    }
}
