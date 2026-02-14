use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiClientError> {
        let url = format!("{}{}", self.base_url, path);
        
        let response = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ApiClientError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiClientError::Http(response.status().as_u16()));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ApiClientError::Deserialize(e.to_string()))
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R, ApiClientError> {
        let url = format!("{}{}", self.base_url, path);
        
        let response = self
            .client
            .post(&url)
            .json(body)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ApiClientError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiClientError::Http(response.status().as_u16()));
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ApiClientError::Deserialize(e.to_string()))
    }

    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R, ApiClientError> {
        let url = format!("{}{}", self.base_url, path);
        
        let response = self
            .client
            .put(&url)
            .json(body)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ApiClientError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiClientError::Http(response.status().as_u16()));
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ApiClientError::Deserialize(e.to_string()))
    }

    pub async fn delete(&self, path: &str) -> Result<(), ApiClientError> {
        let url = format!("{}{}", self.base_url, path);
        
        let response = self
            .client
            .delete(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| ApiClientError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiClientError::Http(response.status().as_u16()));
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("HTTP error: {0}")]
    Http(u16),
    #[error("Deserialize error: {0}")]
    Deserialize(String),
    #[error("Serialize error: {0}")]
    Serialize(String),
}
