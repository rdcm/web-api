use crate::ports::IUserRepository;
use async_trait::async_trait;
use domain::commands::{CreateUserCommand, ICommandHandler};
use domain::queries::{GetUserQuery, IQueryHandler, User};
use std::sync::Arc;

pub struct CreateUserCommandHandler {
    pub repo: Arc<dyn IUserRepository>,
}

#[async_trait]
impl ICommandHandler<CreateUserCommand, Option<String>> for CreateUserCommandHandler {
    async fn handle(&self, cmd: CreateUserCommand) -> Option<String> {
        let user = User {
            id: String::new(),
            name: cmd.name,
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
