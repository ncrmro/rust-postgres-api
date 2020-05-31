extern crate fake;
extern crate planet_express;

mod common;
use actix_web::test::{read_response, TestRequest};
use std::borrow::Borrow;

#[actix_rt::test]
async fn test_index_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let req = TestRequest::with_header("content-type", "text/plain").to_request();
    let res = read_response(&mut srv, req).await;
    assert_eq!(res, "Hello World".as_bytes());
    common::teardown(db_conn, test_name).await;
}

use std::{thread, time};

#[actix_rt::test]
async fn test_user_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::new(db_conn.clone()).await;
    common::teardown(db_conn, test_name).await;
}
