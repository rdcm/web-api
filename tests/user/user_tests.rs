use crate::gen::Gen;
use crate::sut::Sut;

#[actix_rt::test]
async fn user_not_found() {
    let fake_user_id = Gen::random_string();
    let sut = Sut::new().await;

    let user_id = sut.get_user(fake_user_id).await;

    assert_eq!(user_id, "404 Not Found");
}