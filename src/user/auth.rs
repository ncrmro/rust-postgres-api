use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::types::chrono::Utc;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    exp: usize,
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
