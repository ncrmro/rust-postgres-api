extern crate planet_express as src;

mod common;
mod todos;
mod users;

use actix_web::test;

#[actix_rt::test]
async fn test_index_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let res = test::read_response(&mut srv, req).await;
    assert_eq!(std::str::from_utf8(res.as_ref()).unwrap(), "Hello World");
    common::teardown(db_conn, test_name).await;
}
