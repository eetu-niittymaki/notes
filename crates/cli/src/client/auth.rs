use super::ApiClient;

use notes_core::error::Result;
use notes_core::models::auth::{
    AuthResponse,
    LoginRequest,
    RegisterRequest
};

impl ApiClient {
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        let response = self
            .http
            .post(format!("{}auth/login", self.auth_url))
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<AuthResponse>()
            .await?;

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