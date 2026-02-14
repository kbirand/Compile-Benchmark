use axum::{extract::State, Json};

use crate::{
    error::Result,
    models::{ExportFormat, ExportResponse, ExportStatus},
    AppState,
};

pub async fn export_data(State(_state): State<AppState>) -> Result<Json<ExportResponse>> {
    let job_id = uuid::Uuid::new_v4().to_string();

    let response = ExportResponse {
        job_id: job_id.clone(),
        status: ExportStatus::Pending,
        download_url: None,
        expires_at: None,
    };

    // In production, this would queue an async export job
    tracing::info!(job_id = %job_id, "Export job created");

    Ok(Json(response))
}

pub fn generate_csv_export<T: serde::Serialize>(
    data: &[T],
    fields: &[String],
) -> Result<String> {
    let mut csv = String::new();

    // Header row
    csv.push_str(&fields.join(","));
    csv.push('\n');

    // Data rows (simplified - would use proper CSV library)
    for item in data {
        if let Ok(json) = serde_json::to_value(item) {
            let row: Vec<String> = fields
                .iter()
                .map(|f| {
                    json.get(f)
                        .map(|v| v.to_string().trim_matches('"').to_string())
                        .unwrap_or_default()
                })
                .collect();
            csv.push_str(&row.join(","));
            csv.push('\n');
        }
    }

    Ok(csv)
}

pub fn determine_export_format(format_str: &str) -> ExportFormat {
    match format_str.to_lowercase().as_str() {
        "csv" => ExportFormat::Csv,
        "json" => ExportFormat::Json,
        "xlsx" => ExportFormat::Xlsx,
        "pdf" => ExportFormat::Pdf,
        _ => ExportFormat::Csv,
    }
}
