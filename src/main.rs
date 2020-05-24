#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/auth")]
fn authenticate() -> &'static str {
    "Authenticate"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, authenticate])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_hello() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
    #[test]
    fn test_auth() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/auth").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Authenticate".into()));
    }
}
