pub mod contracts_module {
    #[derive(Deserialize)]
    pub struct UserRequest {
        pub name: String,
        pub age: u8,
        pub id: u8,
    }
}