use crate::db::user_db::find_user_by_email;
use crate::db_entities::user_entity::UserEntity;
use crate::json::credentials::Credentials;
use tokio_postgres::Error;

pub async fn login(credentials: Credentials) -> Result<UserEntity, Error> {
    find_user_by_email(credentials.email).await
}
