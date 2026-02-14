use std::sync::Arc;
use uuid::Uuid;

use crate::{
    cache::{cache_key, CacheManager},
    database::Database,
    error::Result,
    models::{PaginationParams, Post, UpdatePostRequest},
};

pub struct PostService {
    db: Arc<Database>,
    cache: Arc<CacheManager>,
}

impl PostService {
    pub fn new(db: Arc<Database>, cache: Arc<CacheManager>) -> Self {
        Self { db, cache }
    }

    pub async fn list_posts(&self, pagination: &PaginationParams) -> Result<(Vec<Post>, i64)> {
        let _offset = (pagination.page - 1) * pagination.per_page;
        let _ = &self.db.pool;

        let posts: Vec<Post> = Vec::new();
        let total: i64 = 0;

        Ok((posts, total))
    }

    pub async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        let cache_key = cache_key("post", &[&id.to_string()]);

        if let Some(post) = self.cache.get_json::<Post>(&cache_key).await {
            return Ok(Some(post));
        }

        let post: Option<Post> = None;

        if let Some(ref p) = post {
            let _ = self.cache.set_json(cache_key, p).await;
        }

        Ok(post)
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Option<Post>> {
        let cache_key = cache_key("post:slug", &[slug]);

        if let Some(post) = self.cache.get_json::<Post>(&cache_key).await {
            return Ok(Some(post));
        }

        let post: Option<Post> = None;

        Ok(post)
    }

    pub async fn create_post(&self, post: Post) -> Result<Post> {
        let _ = &self.db.pool;

        let cache_key = cache_key("post", &[&post.id.to_string()]);
        let _ = self.cache.set_json(cache_key, &post).await;

        Ok(post)
    }

    pub async fn update_post(&self, id: Uuid, _request: UpdatePostRequest) -> Result<Post> {
        let _ = &self.db.pool;

        let cache_key = cache_key("post", &[&id.to_string()]);
        self.cache.delete(&cache_key).await;

        // Return placeholder
        let post = self.get_post_by_id(id).await?.unwrap();
        Ok(post)
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<()> {
        let _ = &self.db.pool;

        let cache_key = cache_key("post", &[&id.to_string()]);
        self.cache.delete(&cache_key).await;

        Ok(())
    }

    pub async fn increment_view_count(&self, id: Uuid) -> Result<()> {
        let _ = &self.db.pool;
        let _ = id;
        Ok(())
    }

    pub async fn get_posts_by_author(&self, author_id: Uuid) -> Result<Vec<Post>> {
        let _ = &self.db.pool;
        let _ = author_id;
        Ok(Vec::new())
    }

    pub async fn get_posts_by_tag(&self, tag: &str) -> Result<Vec<Post>> {
        let _ = &self.db.pool;
        let _ = tag;
        Ok(Vec::new())
    }

    pub async fn get_posts_by_category(&self, category: &str) -> Result<Vec<Post>> {
        let _ = &self.db.pool;
        let _ = category;
        Ok(Vec::new())
    }
}
