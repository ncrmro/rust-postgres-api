use crate::core::types::io::get_current_user::*;
use crate::core::types::io::login_user::*;
use crate::core::types::io::register_user::*;
use crate::core::types::user::CurrentUser;
use crate::core::usecases::get_current_user::*;
use crate::core::usecases::login_user::*;
use crate::core::usecases::register_user::*;
use crate::db::{DbConn, PGTokenRepo, PGUserRepo};
use crate::http::api::ApiResult;
use crate::settings::Settings;
use rocket::State;

#[get("/", format = "application/json")]
fn current_user_handler(
    current_user: CurrentUser,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<CurrentUserOutput, CurrentUserError> {
    println!("{:?}", current_user);
    let user_repo = PGUserRepo::new(&db);
    ApiResult(get_current_user(current_user, &user_repo))
}

#[post("/", format = "application/json", data = "<register_user_input>")]
fn register_user_handler(
    register_user_input: Result<RegisterUserInput, RegisterUserError>,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<RegisterUserOutput, RegisterUserError> {
    if let Err(err) = register_user_input {
        return ApiResult(Err(err));
    }

    let user_repo = PGUserRepo::new(&db);
    ApiResult(register_user(register_user_input.unwrap(), &user_repo))
}

#[post("/login", format = "application/json", data = "<login_user_input>")]
fn login_user_handler(
    login_user_input: Result<LoginUserInput, LoginUserError>,
    db: DbConn,
    settings: State<Settings>,
) -> ApiResult<LoginUserOutput, LoginUserError> {
    if let Err(err) = login_user_input {
        return ApiResult(Err(err));
    }

    let user_repo = PGUserRepo::new(&db);
    let token_repo = PGTokenRepo::new(&db, &settings.auth);

    ApiResult(login_user(
        login_user_input.unwrap(),
        &user_repo,
        &token_repo,
    ))
}
