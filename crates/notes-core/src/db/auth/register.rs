use libsql::Connection;

use crate::error::Result;

use crate::db::users::create;
use crate::models::user::User;
use crate::db::auth::hash_password::hash_password;

pub async fn register(
    conn: &Connection,
    username: String,
    password: String
) -> Result<User> {
    let password_hash = hash_password(&password)?;
    
    let result = create::create(
        conn,
        &username,
        &password_hash,
    ).await;

    let user = match result {
        Ok(user) => user,

        Err(e) if e.to_string().contains("UNIQUE constraint failed: users.username") => {
            return Err(crate::error::Error::UserAlreadyExists);
        }

        Err(e) => return Err(e),
    };

    Ok(user)
}