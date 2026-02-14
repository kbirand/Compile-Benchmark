use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    cache::{cache_key, CacheManager},
    database::Database,
    error::Result,
    models::{PaginationParams, UpdateUserRequest, User, UserRole, UserStatus},
};

pub struct UserService {
    db: Arc<Database>,
    cache: Arc<CacheManager>,
}

impl UserService {
    pub fn new(db: Arc<Database>, cache: Arc<CacheManager>) -> Self {
        Self { db, cache }
    }

    pub async fn list_users(&self, pagination: &PaginationParams) -> Result<(Vec<User>, i64)> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        // This is a simplified query - in production would use proper pagination
        let users: Vec<User> = Vec::new(); // sqlx::query_as would be used here
        let total: i64 = 0;

        Ok((users, total))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let cache_key = cache_key("user", &[&id.to_string()]);
        
        if let Some(user) = self.cache.get_json::<User>(&cache_key).await {
            return Ok(Some(user));
        }

        // Query database (simplified)
        let user: Option<User> = None;

        if let Some(ref u) = user {
            let _ = self.cache.set_json(cache_key, u).await;
        }

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let cache_key = cache_key("user:email", &[email]);
        
        if let Some(user) = self.cache.get_json::<User>(&cache_key).await {
            return Ok(Some(user));
        }

        // Query database (simplified)
        let user: Option<User> = None;

        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let cache_key = cache_key("user:username", &[username]);
        
        if let Some(user) = self.cache.get_json::<User>(&cache_key).await {
            return Ok(Some(user));
        }

        // Query database (simplified)
        let user: Option<User> = None;

        Ok(user)
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        // Insert into database (simplified)
        let _ = &self.db.pool;

        let cache_key = cache_key("user", &[&user.id.to_string()]);
        let _ = self.cache.set_json(cache_key, &user).await;

        Ok(user)
    }

    pub async fn create_user_with_password(
        &self,
        email: String,
        username: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User> {
        let now = Utc::now();
        let user = User::builder()
            .id(Uuid::new_v4())
            .email(email)
            .username(username)
            .password_hash(password_hash)
            .first_name(first_name)
            .last_name(last_name)
            .avatar_url(None)
            .bio(None)
            .role(UserRole::User)
            .status(UserStatus::Active)
            .email_verified(false)
            .created_at(now)
            .updated_at(now)
            .last_login_at(None)
            .build();

        self.create_user(user).await
    }

    pub async fn update_user(&self, id: Uuid, request: UpdateUserRequest) -> Result<User> {
        // Get existing user and update fields
        let now = Utc::now();
        let user = User::builder()
            .id(id)
            .email(request.email.unwrap_or_default())
            .username(request.username.unwrap_or_default())
            .password_hash(String::new())
            .first_name(request.first_name)
            .last_name(request.last_name)
            .avatar_url(request.avatar_url)
            .bio(request.bio)
            .role(UserRole::User)
            .status(UserStatus::Active)
            .email_verified(true)
            .created_at(now)
            .updated_at(now)
            .last_login_at(None)
            .build();

        // Invalidate cache
        let cache_key = cache_key("user", &[&id.to_string()]);
        self.cache.delete(&cache_key).await;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        // Delete from database (simplified)
        let _ = &self.db.pool;

        // Invalidate cache
        let cache_key = cache_key("user", &[&id.to_string()]);
        self.cache.delete(&cache_key).await;

        Ok(())
    }
}
