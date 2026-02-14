use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ExternalApiService {
    client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest<T> {
    pub endpoint: String,
    pub method: HttpMethod,
    pub body: Option<T>,
    pub query_params: Option<Vec<(String, String)>>,
    pub headers: Option<Vec<(String, String)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: Option<T>,
    pub error: Option<ApiErrorResponse>,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ExternalApiService {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ExternalApiError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| ExternalApiError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ExternalApiError::HttpError(response.status().as_u16()));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ExternalApiError::ParseError(e.to_string()))
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, ExternalApiError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| ExternalApiError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ExternalApiError::HttpError(response.status().as_u16()));
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ExternalApiError::ParseError(e.to_string()))
    }

    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, ExternalApiError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| ExternalApiError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ExternalApiError::HttpError(response.status().as_u16()));
        }

        response
            .json::<R>()
            .await
            .map_err(|e| ExternalApiError::ParseError(e.to_string()))
    }

    pub async fn delete(&self, endpoint: &str) -> Result<(), ExternalApiError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| ExternalApiError::RequestError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ExternalApiError::HttpError(response.status().as_u16()));
        }

        Ok(())
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExternalApiError {
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("HTTP error: {0}")]
    HttpError(u16),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Timeout")]
    Timeout,
    #[error("Rate limited")]
    RateLimited,
}

// Example external API models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub location: String,
    pub temperature: f64,
    pub humidity: f64,
    pub conditions: String,
    pub forecast: Vec<ForecastDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub high: f64,
    pub low: f64,
    pub conditions: String,
    pub precipitation_chance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub formatted_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place_id: String,
    pub types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyExchangeRate {
    pub base: String,
    pub target: String,
    pub rate: f64,
    pub timestamp: i64,
}
