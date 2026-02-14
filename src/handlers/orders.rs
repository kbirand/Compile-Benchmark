use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{CreateOrderRequest, OrderListResponse, OrderResponse, PaginationParams},
    services::order_service::OrderService,
    AppState,
};

pub async fn list_orders(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<OrderListResponse>> {
    let service = OrderService::new(state.db.clone(), state.cache.clone());
    let (orders, total) = service.list_orders(&pagination).await?;

    let response = OrderListResponse {
        orders,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
    };

    Ok(Json(response))
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>> {
    let service = OrderService::new(state.db.clone(), state.cache.clone());
    let order = service.get_order_by_id(id).await?;

    match order {
        Some(o) => Ok(Json(o)),
        None => Err(AppError::NotFound(format!("Order {} not found", id))),
    }
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>> {
    let service = OrderService::new(state.db.clone(), state.cache.clone());
    let order = service.create_order(request).await?;
    Ok(Json(order))
}
