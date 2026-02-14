use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TypedBuilder)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub status: PostStatus,
    pub visibility: PostVisibility,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub view_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "post_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "post_visibility", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PostVisibility {
    Public,
    Private,
    Unlisted,
    MembersOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TypedBuilder)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be 1-200 characters"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    #[validate(length(max = 500))]
    pub excerpt: Option<String>,
    #[validate(url)]
    pub featured_image_url: Option<String>,
    pub status: Option<PostStatus>,
    pub visibility: Option<PostVisibility>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TypedBuilder)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    pub content: Option<String>,
    #[validate(length(max = 500))]
    pub excerpt: Option<String>,
    #[validate(url)]
    pub featured_image_url: Option<String>,
    pub status: Option<PostStatus>,
    pub visibility: Option<PostVisibility>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub status: PostStatus,
    pub visibility: PostVisibility,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub view_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            title: post.title,
            slug: post.slug,
            content: post.content,
            excerpt: post.excerpt,
            featured_image_url: post.featured_image_url,
            status: post.status,
            visibility: post.visibility,
            tags: post.tags,
            categories: post.categories,
            view_count: post.view_count,
            like_count: post.like_count,
            comment_count: post.comment_count,
            published_at: post.published_at,
            created_at: post.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostListResponse {
    pub posts: Vec<PostResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub like_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
