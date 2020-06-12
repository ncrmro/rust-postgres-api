use super::super::common;
use actix_web::test;
use planet_express::user::model::User;
use planet_express::user::routes::AuthenticationResponse;

#[actix_rt::test]
async fn test_auth_viewer_create() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::build();
    let req = test::TestRequest::post()
        .uri("/v1/viewer/create")
        .set_json(&obj)
        .to_request();
    let res: AuthenticationResponse = test::read_response_json(&mut srv, req).await;

    assert_eq!(res.user.email, obj.email);

    common::teardown(db_conn, test_name).await;
}

#[actix_rt::test]
async fn test_auth_viewer_authenticate() {
    let (mut srv, db_conn, test_name) = common::setup().await;
    let obj = planet_express::user::UserFactory::create(db_conn.clone()).await;
    let req = test::TestRequest::post()
        .uri("/v1/viewer/authenticate")
        .set_json(&obj)
        .to_request();

    let res: AuthenticationResponse = test::read_response_json(&mut srv, req).await;
    assert_eq!(res.user.email, obj.email.clone());

    let req = test::TestRequest::get()
        .uri("/v1/viewer")
        .header("Authorization", format!("Bearer {}", res.token))
        .to_request();
    let res: User = test::read_response_json(&mut srv, req).await;
    assert_eq!(res.email, obj.email.clone());
    common::teardown(db_conn, test_name).await;
}
