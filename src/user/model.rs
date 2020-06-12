use fake::{faker::internet, Fake};

pub use crate::core::auth::NewUser;
pub use crate::core::auth::User;
use crate::core::auth::ViewerModel;
use crate::core::db::PgPool;

pub struct UserFactory;

impl UserFactory {
    pub fn build() -> NewUser {
        NewUser {
            email: internet::en::FreeEmail().fake(),
            password: "testpassword".to_string(),
        }
    }

    pub async fn create(pool: PgPool) -> User {
        let obj_build = UserFactory::build();
        User::create_user(obj_build, &pool).await.unwrap()
    }
}
