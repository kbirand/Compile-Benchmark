use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaginationParams {
    #[builder(default = 1)]
    pub page: i32,
    #[builder(default = 20)]
    pub per_page: i32,
    #[builder(default)]
    pub sort_by: Option<String>,
    #[builder(default)]
    pub sort_order: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub current_page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i32,
    pub has_previous: bool,
    pub has_next: bool,
}

impl PaginationMeta {
    pub fn new(current_page: i32, per_page: i32, total_items: i64) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as i32;
        Self {
            current_page,
            per_page,
            total_items,
            total_pages,
            has_previous: current_page > 1,
            has_next: current_page < total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: Option<String>,
    pub filters: Option<Vec<FilterParam>>,
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterParam {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub metadata: Option<ResponseMetadata>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: ResponseMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
    pub uptime_seconds: u64,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size: u64,
    pub url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub filters: Option<Vec<FilterParam>>,
    pub fields: Option<Vec<String>>,
    pub include_headers: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
    Xlsx,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResponse {
    pub job_id: String,
    pub status: ExportStatus,
    pub download_url: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}
