mod gen;
mod sut;

use api::models::TrackActivityRequest;
use domain::events::ActivityEvent;

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

    let created_user = sut.create_user(name.to_string(), age).await.unwrap();

    let user_id = created_user.id;

    let user = sut.get_user(user_id.clone()).await.unwrap();

    assert_eq!(user.id, user_id);
    assert_eq!(user.name, name);
    assert_eq!(user.age, age);
}

#[actix_rt::test]
async fn open_tracked() {
    let path = Gen::random_string();
    let sut = Sut::new().await;

    sut.track_activity(&TrackActivityRequest::Open { path })
        .await
        .unwrap();

    let event = sut.get_activity_event();

    assert!(matches!(event, ActivityEvent::Open { p: path }));
}

#[actix_rt::test]
async fn click_tracked() {
    let x = Gen::random_i32();
    let y = Gen::random_i32();
    let path = Gen::random_string();
    let sut = Sut::new().await;

    sut.track_activity(&TrackActivityRequest::Click { x, y })
        .await
        .unwrap();

    let event = sut.get_activity_event();

    assert!(matches!(event, ActivityEvent::Click { x, y }));
}
