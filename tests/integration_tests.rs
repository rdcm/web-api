mod sut;

#[cfg(test)]
mod integration_tests {
    use crate::sut::sut::Sut;

    #[test]
    fn it_works() {
        let sut = Sut::create();
        sut.create_user();

        let user = sut.get_user();
    }
}