use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatedUserIdResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub name: String,
    pub age: u8,
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub code: i32,
}
