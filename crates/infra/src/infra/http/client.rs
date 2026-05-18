use crate::infra::http::error::HttpError;

#[derive(Clone, Debug, Default)]
pub struct HttpClient {
    pub base_url: String,
}

impl HttpClient {
    pub fn get(&self, path: &str) -> Result<String, HttpError> {
        if path.trim().is_empty() {
            return Err(HttpError::InvalidRequest(
                "path cannot be empty".to_string(),
            ));
        }
        Ok(format!("GET {}{}", self.base_url, path))
    }
}
