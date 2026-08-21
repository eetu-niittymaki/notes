pub struct ApiClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }
}