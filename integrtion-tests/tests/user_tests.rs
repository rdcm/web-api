mod gen;
mod sut;

use crate::gen::Gen;
use crate::sut::Sut;

#[actix_rt::test]
async fn user_not_found() {
    let fake_user_id = Gen::random_string();
    let sut = Sut::new().await;

    let user = sut.get_user(fake_user_id).await;

    assert!(user.is_err());
}

#[actix_rt::test]
async fn user_created() {
    let name = Gen::random_string();
    let age = Gen::random_u8();

    let sut = Sut::new().await;

    let created_user = sut
        .clone()
        .create_user(name.to_string(), age)
        .await
        .unwrap();

    let user_id = created_user.id;

    let user = sut.clone().get_user(user_id.clone()).await.unwrap();

    assert_eq!(user.id, user_id);
    assert_eq!(user.name, name);
    assert_eq!(user.age, age);
}
