#[cfg(test)]
mod test {
    use planet_express::http::api::rocket_init;
    use planet_express::settings::Settings;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let settings = Settings::new().unwrap();
        let client = Client::new(rocket_init(settings)).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
