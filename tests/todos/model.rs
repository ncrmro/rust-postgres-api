use super::super::common;
use src::core::db::model::DatabaseModel;
use src::core::factory::ModelFactory;
use src::todo::factory;
use src::todo::model;

#[actix_rt::test]
async fn test_model_create() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = factory::Todo::build();

    let record = model::Todo::create(obj.clone(), &db_conn).await.unwrap();

    assert_eq!(obj.description, record.description);
    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_model_get_by_id() {
    let (_srv, db_conn, test_name) = common::setup().await;
    let obj = factory::Todo::create(None, db_conn.clone()).await.unwrap();
    let record = model::Todo::get(obj.id, &db_conn).await.unwrap();

    assert_eq!(obj.description, record.description);
    common::teardown(db_conn, test_name).await;
}
