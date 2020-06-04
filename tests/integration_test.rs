// extern crate fake;
extern crate planet_express;

mod common;
mod users;

// use actix_web::http::StatusCode;
use actix_web::test;
use actix_web::test::{read_response, TestRequest};
use planet_express::user::{AuthResponse, User};

#[actix_rt::test]
async fn test_index_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let req = TestRequest::with_header("content-type", "text/plain").to_request();
    let res = read_response(&mut srv, req).await;
    assert_eq!(std::str::from_utf8(res.as_ref()).unwrap(), "Hello World");
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_auth_viewer_create() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::build();
    let req = TestRequest::post()
        .uri("/v1/viewer/create")
        .set_json(&obj)
        .to_request();
    let res: AuthResponse = test::read_response_json(&mut srv, req).await;

    assert_eq!(res.user.email, obj.email);

    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_auth_viewer_authenticate() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::create(db_conn.clone()).await;
    let req = TestRequest::post()
        .uri("/v1/viewer/authenticate")
        .set_json(&obj)
        .to_request();

    let res: AuthResponse = test::read_response_json(&mut srv, req).await;
    assert_eq!(res.user.email, obj.email.clone());

    let req = TestRequest::get()
        .uri("/v1/viewer")
        .header("Authorization", format!("Bearer {}", res.token))
        .to_request();

    let res: User = test::read_response_json(&mut srv, req).await;
    assert_eq!(res.email, obj.email.clone());

    common::teardown(db_conn, test_name).await;
}
