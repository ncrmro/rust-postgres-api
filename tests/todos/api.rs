use super::super::common;
use actix_web::test;
use src::core::factory::ModelFactory;
use src::todo::factory;
use src::todo::model;

#[actix_rt::test]
async fn test_get() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = factory::Todo::create(None, db_conn.clone()).await.unwrap();
    let req = test::TestRequest::get()
        .uri(format!("/v1/todo/{}", obj.id).as_ref())
        .to_request();

    let res: model::Todo = test::read_response_json(&mut srv, req).await;
    assert_eq!(res.description, obj.description);
    common::teardown(db_conn, test_name).await;
}
