extern crate planet_express;
mod common;

use actix_web::test;

#[actix_rt::test]
async fn test_index_get() {
    let mut server = common::server().await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut server, req).await;
    assert!(resp.status().is_success());
}
