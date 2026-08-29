use super::ApiClient;

use notes_core::error::{Result, Error};
use notes_core::models::auth::{
    AuthResponse,
    LoginRequest,
    RegisterRequest
};

impl ApiClient {
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, Error> {
        let response = self
            .http
            .post(format!("{}auth/login", self.auth_url))
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if status.is_client_error() {
            return Err(Error::LoginError);
        }

        let body = response.text().await?;

        let response: AuthResponse = serde_json::from_str(&body)?;
        Ok(response)
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse> {
        let response = self
            .http
            .post(format!("{}auth/register", self.auth_url))
            .json(&request)
            .send()
            .await?;

        let body = response.text().await?;

        let response: AuthResponse = serde_json::from_str(&body)?;
        Ok(response)
    }
}