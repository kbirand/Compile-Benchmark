use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{
        CreateUserRequest, PaginationParams, UpdateUserRequest, User, UserListResponse,
        UserResponse, UserRole, UserStatus,
    },
    services::user_service::UserService,
    AppState,
};

pub async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<UserListResponse>> {
    let service = UserService::new(state.db.clone(), state.cache.clone());
    let (users, total) = service.list_users(&pagination).await?;

    let response = UserListResponse {
        users: users.into_iter().map(UserResponse::from).collect(),
        total,
        page: pagination.page,
        per_page: pagination.per_page,
    };

    Ok(Json(response))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let service = UserService::new(state.db.clone(), state.cache.clone());
    let user = service.get_user_by_id(id).await?;

    match user {
        Some(u) => Ok(Json(UserResponse::from(u))),
        None => Err(AppError::NotFound(format!("User {} not found", id))),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    let service = UserService::new(state.db.clone(), state.cache.clone());
    
    // Check if email already exists
    if service.get_user_by_email(&request.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Check if username already exists
    if service.get_user_by_username(&request.username).await?.is_some() {
        return Err(AppError::Conflict("Username already taken".to_string()));
    }

    let now = Utc::now();
    let user = User::builder()
        .id(Uuid::new_v4())
        .email(request.email)
        .username(request.username)
        .password_hash("hashed".to_string()) // Would properly hash in production
        .first_name(request.first_name)
        .last_name(request.last_name)
        .avatar_url(None)
        .bio(None)
        .role(UserRole::User)
        .status(UserStatus::Active)
        .email_verified(false)
        .created_at(now)
        .updated_at(now)
        .last_login_at(None)
        .build();

    let created = service.create_user(user).await?;
    Ok(Json(UserResponse::from(created)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    let service = UserService::new(state.db.clone(), state.cache.clone());
    
    let existing = service.get_user_by_id(id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!("User {} not found", id)));
    }

    let updated = service.update_user(id, request).await?;
    Ok(Json(UserResponse::from(updated)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let service = UserService::new(state.db.clone(), state.cache.clone());
    
    let existing = service.get_user_by_id(id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!("User {} not found", id)));
    }

    service.delete_user(id).await?;
    Ok(Json(serde_json::json!({ "deleted": true, "id": id })))
}
