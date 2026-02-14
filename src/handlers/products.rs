use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{PaginationParams, ProductListResponse, ProductResponse},
    services::product_service::ProductService,
    AppState,
};

pub async fn list_products(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ProductListResponse>> {
    let service = ProductService::new(state.db.clone(), state.cache.clone());
    let (products, total) = service.list_products(&pagination).await?;

    let response = ProductListResponse {
        products: products.into_iter().map(ProductResponse::from).collect(),
        total,
        page: pagination.page,
        per_page: pagination.per_page,
    };

    Ok(Json(response))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductResponse>> {
    let service = ProductService::new(state.db.clone(), state.cache.clone());
    let product = service.get_product_by_id(id).await?;

    match product {
        Some(p) => Ok(Json(ProductResponse::from(p))),
        None => Err(AppError::NotFound(format!("Product {} not found", id))),
    }
}
