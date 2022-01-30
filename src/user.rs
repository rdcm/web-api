pub mod user_module {
    trait IUserRepository {
        fn create(&self, user: User);
        fn get(&self, id: u8) -> User;
    }

    pub struct User {
        pub name: String,
        pub age: u8,
        pub id: u8,
    }
}
