use crate::core::settings::Settings;
use argon2::{self, Config};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::types::chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

pub fn jwt_get(user_id: i32) -> String {
    encode(
        &Header::default(),
        &Claims {
            user_id,
            exp: (Utc::now().timestamp() + 10000) as usize,
        },
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

pub fn jwt_verify(token: String) -> Result<Claims, bool> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );
    match token {
        Ok(token_data) => Ok(token_data.claims),
        Err(_err) => Err(false),
    }
}

pub fn hash_password(password: String) -> String {
    // todo maybe pass this settings in somehow?
    let settings = Settings::new().unwrap();
    let salt = settings.auth.password_salt.as_ref();
    let config = Config::default();
    argon2::hash_encoded(password.as_ref(), salt, &config).unwrap()
}

pub fn verify_password(hashed_password: String, password: String) -> bool {
    argon2::verify_encoded(hashed_password.as_ref(), password.as_ref()).is_ok()
}
