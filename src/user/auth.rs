use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::types::chrono::Utc;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
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
