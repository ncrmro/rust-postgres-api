use chrono::Local;
use core::types::token::*;
use core::types::user::*;
use core::types::RepoError;
use db::pool::*;
use jwt::{encode, Algorithm};
use settings::Auth;

pub struct PGTokenRepo<'a> {
    db_conn: &'a DbConn,
    settings: &'a Auth,
}

impl<'a> PGTokenRepo<'a> {
    pub fn new(conn: &'a DbConn, settings: &'a Auth) -> PGTokenRepo<'a> {
        PGTokenRepo {
            db_conn: conn,
            settings,
        }
    }
}

impl<'a> TokenRepo for PGTokenRepo<'a> {
    fn create_login_token(&self, user: &User) -> Result<String, RepoError> {
        let payload = json!({
            "iss" : self.settings.issuer,
            "sub" : user.username,
            "iat" : Local::now().timestamp(),
            "exp" : Local::now().timestamp() + self.settings.expiry
        });

        let header = json!({});
        encode(header, &self.settings.secret, &payload, Algorithm::HS256)
            .map_err(|e| RepoError::from(e))
    }
}
