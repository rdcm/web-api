use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CreatedUserIdResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub name: String,
    pub age: u8,
    pub id: String,
}
