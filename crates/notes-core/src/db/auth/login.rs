use libsql::Connection;

use crate::error::Result;
use crate::db::users::get::by_username;
use crate::models::user::User;
use crate::db::auth::hash_password::verify_password;

pub async fn login(
    conn: &Connection,
    username: String,
    password: String,
) -> Result<Option<User>> {
    let user = by_username(conn, &username).await?;

    match user {
        Some(user) => {
            if verify_password(&password, &user.password_hash).is_ok() {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }

        None => Ok(None),
    }
}