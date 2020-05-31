use planet_express::settings::Settings;
use std::panic;

async fn setup() {
    let settings = Settings::new().unwrap();
    super::db::init(settings).await;
}

async fn teardown() {
    let settings = Settings::new().unwrap();
    super::db::down(settings).await;
}

pub async fn run_test<T>(test: T) -> ()
where
    T: FnOnce(String) -> () + panic::UnwindSafe,
{
    setup().await;
    // let srv = super::server().await;

    let text = "text";
    let result = panic::catch_unwind(|| test(text.parse().unwrap()));

    teardown().await;

    assert!(result.is_ok())
}
