pub mod create;
pub mod get;
pub mod delete;

use libsql::Connection;

use crate::error::Result;

use crate::models::user::{
    User,
    NewUser,
    DeleteUser,
};

pub struct UsersRepository<'a> {
    conn: &'a Connection,
}

impl<'a> UsersRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn create(&self, user: NewUser) -> Result<User> {
        create::create(self.conn, &user.username, &user.password_hash).await
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>> {
        get::by_id(self.conn, id).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        get::by_username(self.conn, username).await
    }

    pub async fn delete(&self, delete: DeleteUser) -> Result<u64> {
        delete::delete(self.conn, delete.id).await
    }
}