use reqwest::{RequestBuilder, Response};
use notes_core::error::{Error, Result};

use crate::auth::credential_manager;

pub struct ApiClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) auth_url: String,
    pub(crate) token: Option<String>,
}

impl ApiClient {
    pub fn new(
        base_url: impl Into<String>, 
        auth_url: impl Into<String>, 
        token: Option<String>
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            auth_url: auth_url.into(),
            token,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    fn authorize(
        &self,
        request: RequestBuilder,
    ) -> RequestBuilder {
        match &self.token {
            Some(token) => request.bearer_auth(token),
            None => request,
        }
    }

    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let request = self
            .http
            .get(format!("{}{}", self.base_url, path));

        self.authorize(request)
    }

    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        let request = self
            .http
            .post(format!("{}{}", self.base_url, path));

        self.authorize(request)
    }

    pub(crate) fn patch(&self, path: &str) -> RequestBuilder {
        let request = self
            .http
            .patch(format!("{}{}", self.base_url, path));

        self.authorize(request)
    }

    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        let request = self
            .http
            .delete(format!("{}{}", self.base_url, path));

        self.authorize(request)
    }

    fn check_response(response: Response) -> Result<Response> {
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                credential_manager::delete_tokens()?;
                Err(Error::Unauthorized)
            }

            reqwest::StatusCode::CONFLICT => {
                Err(Error::UserAlreadyExists)
            }

            _ => Ok(response.error_for_status()?),
        }
    }

    pub(crate) async fn send(
        &self,
        request: RequestBuilder,
    ) -> Result<Response> {
        let response = request.send().await?;

        Self::check_response(response)
    }
}