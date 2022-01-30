pub mod contracts_module {
    #[derive(Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub age: u8
    }
    
    #[derive(Serialize)]
    pub struct UserResponse {
        pub name: String,
        pub age: u8,
        pub id: u32,
    }

    #[derive(Serialize)]
    pub struct UserIdResponse {
        pub id: u8,
    }
}