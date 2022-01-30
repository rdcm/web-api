mod dal {
    impl IUserRepository for UserRepository {
        fn create(&self, user: User) {
    
        }
        fn get(&self, id: u8) -> User;
    }
}