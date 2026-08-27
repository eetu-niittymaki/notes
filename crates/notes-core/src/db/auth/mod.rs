pub mod hash_password;

mod login;
mod register;

use libsql::Connection;
use crate::error::Result;

use crate::models::auth::{
    LoginRequest,
    RegisterRequest
};
use crate::models::user::User;

pub struct AuthService<'a> {
    conn: &'a Connection,
}

impl<'a> AuthService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn login(
        &self, 
        login: LoginRequest, 
    ) -> Result<Option<User>> {
        login::login(
            self.conn, 
            login.username, 
            login.password, 
        ).await
    }

    pub async fn register(
        &self, 
        register: RegisterRequest, 
    ) -> Result<User> {
        register::register(
            self.conn, 
            register.username, 
            register.password, 
        ).await
    }
}