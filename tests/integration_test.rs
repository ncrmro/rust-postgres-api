extern crate fake;
extern crate planet_express;

mod common;
use actix_web::test;
use actix_web::test::{read_response, TestRequest};
use planet_express::user::User;

#[actix_rt::test]
async fn test_index_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let req = TestRequest::with_header("content-type", "text/plain").to_request();
    let res = read_response(&mut srv, req).await;
    assert_eq!(res, "Hello World".as_bytes());
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_user_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::build();
    let req = TestRequest::post()
        .uri("/v1/viewer/create")
        .set_json(&obj)
        .to_request();
    let res: User = test::read_response_json(&mut srv, req).await;

    assert_eq!(res.email, obj.email);

    common::teardown(db_conn, test_name).await;
}
