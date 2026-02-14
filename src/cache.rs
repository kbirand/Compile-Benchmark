use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub struct CacheManager {
    string_cache: Cache<String, String>,
    json_cache: Cache<String, String>,
}

impl CacheManager {
    pub fn new() -> Self {
        let string_cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .time_to_idle(Duration::from_secs(60))
            .build();

        let json_cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .time_to_idle(Duration::from_secs(60))
            .build();

        Self {
            string_cache,
            json_cache,
        }
    }

    pub async fn get_string(&self, key: &str) -> Option<String> {
        self.string_cache.get(key).await
    }

    pub async fn set_string(&self, key: String, value: String) {
        self.string_cache.insert(key, value).await;
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        if let Some(json_str) = self.json_cache.get(key).await {
            serde_json::from_str(&json_str).ok()
        } else {
            None
        }
    }

    pub async fn set_json<T: Serialize>(&self, key: String, value: &T) -> Result<(), serde_json::Error> {
        let json_str = serde_json::to_string(value)?;
        self.json_cache.insert(key, json_str).await;
        Ok(())
    }

    pub async fn delete(&self, key: &str) {
        self.string_cache.invalidate(key).await;
        self.json_cache.invalidate(key).await;
    }

    pub async fn clear(&self) {
        self.string_cache.invalidate_all();
        self.json_cache.invalidate_all();
    }

    pub fn entry_count(&self) -> u64 {
        self.string_cache.entry_count() + self.json_cache.entry_count()
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn cache_key(prefix: &str, parts: &[&str]) -> String {
    let mut key = prefix.to_string();
    for part in parts {
        key.push(':');
        key.push_str(part);
    }
    key
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub ttl_seconds: u64,
    pub idle_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            ttl_seconds: 300,
            idle_seconds: 60,
        }
    }
}
