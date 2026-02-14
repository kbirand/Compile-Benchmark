use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{
        CreatePostRequest, PaginationParams, Post, PostListResponse, PostResponse, PostStatus,
        PostVisibility, UpdatePostRequest,
    },
    services::post_service::PostService,
    AppState,
};

pub async fn list_posts(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PostListResponse>> {
    let service = PostService::new(state.db.clone(), state.cache.clone());
    let (posts, total) = service.list_posts(&pagination).await?;

    let response = PostListResponse {
        posts: posts.into_iter().map(PostResponse::from).collect(),
        total,
        page: pagination.page,
        per_page: pagination.per_page,
    };

    Ok(Json(response))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>> {
    let service = PostService::new(state.db.clone(), state.cache.clone());
    let post = service.get_post_by_id(id).await?;

    match post {
        Some(p) => Ok(Json(PostResponse::from(p))),
        None => Err(AppError::NotFound(format!("Post {} not found", id))),
    }
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>> {
    let service = PostService::new(state.db.clone(), state.cache.clone());

    let now = Utc::now();
    let slug = generate_slug(&request.title);
    
    let post = Post::builder()
        .id(Uuid::new_v4())
        .author_id(Uuid::new_v4()) // Would get from auth context
        .title(request.title)
        .slug(slug)
        .content(request.content)
        .excerpt(request.excerpt)
        .featured_image_url(request.featured_image_url)
        .status(request.status.unwrap_or(PostStatus::Draft))
        .visibility(request.visibility.unwrap_or(PostVisibility::Public))
        .tags(request.tags.unwrap_or_default())
        .categories(request.categories.unwrap_or_default())
        .view_count(0)
        .like_count(0)
        .comment_count(0)
        .published_at(request.published_at)
        .created_at(now)
        .updated_at(now)
        .build();

    let created = service.create_post(post).await?;
    Ok(Json(PostResponse::from(created)))
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>> {
    let service = PostService::new(state.db.clone(), state.cache.clone());

    let existing = service.get_post_by_id(id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!("Post {} not found", id)));
    }

    let updated = service.update_post(id, request).await?;
    Ok(Json(PostResponse::from(updated)))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let service = PostService::new(state.db.clone(), state.cache.clone());

    let existing = service.get_post_by_id(id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!("Post {} not found", id)));
    }

    service.delete_post(id).await?;
    Ok(Json(serde_json::json!({ "deleted": true, "id": id })))
}

fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
