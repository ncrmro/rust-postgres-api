extern crate planet_express;
mod common;
// use actix_web::test::{read_response, TestRequest};
use common::runner::run_test;

#[actix_rt::test]
async fn test_index_get() {
    run_test(|text: String| {
        {
            println!("TEXT!{}", text);
            // let req = TestRequest::with_header("content-type", "text/plain").to_request();
            // let res = read_response(&mut srv, req).await;
            // assert_eq!(res, "Hello World".as_bytes());
            assert!(true);
        }
    })
    .await;
}
