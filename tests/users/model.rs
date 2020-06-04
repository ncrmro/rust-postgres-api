extern crate fake;
extern crate planet_express;
use super::super::common;
use planet_express::user;

#[actix_rt::test]
async fn test_model_create() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::build();
    let record = user::User::create(obj.clone(), &db_conn).await.unwrap();

    assert_eq!(obj.email, record.email);
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_model_get_by_id() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::create(db_conn.clone()).await;
    let record = user::User::get_by_id(obj.id, &db_conn).await.unwrap();

    assert_eq!(obj.email, record.email);
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_model_get_by_email() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = user::UserFactory::create(db_conn.clone()).await;
    let record = user::User::get_by_email(obj.email.clone(), &db_conn)
        .await
        .unwrap();

    assert_eq!(obj.email, record.email);
    common::teardown(db_conn, test_name).await;
}
