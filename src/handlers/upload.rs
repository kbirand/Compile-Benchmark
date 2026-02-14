use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{error::Result, models::FileUploadResponse, AppState};

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct PresignedUploadResponse {
    pub upload_url: String,
    pub file_id: String,
    pub expires_in: i64,
}

pub async fn upload_file(
    State(_state): State<AppState>,
    Json(request): Json<UploadRequest>,
) -> Result<Json<FileUploadResponse>> {
    let file_id = uuid::Uuid::new_v4().to_string();
    let filename = format!("{}_{}", file_id, sanitize_filename(&request.filename));

    let response = FileUploadResponse {
        id: file_id.clone(),
        filename: filename.clone(),
        original_filename: request.filename,
        content_type: request.content_type,
        size: request.size,
        url: format!("/uploads/{}", filename),
        thumbnail_url: None,
    };

    Ok(Json(response))
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
