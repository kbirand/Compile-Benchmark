#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod api;
mod auth;
mod cache;
mod config;
mod database;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod templates;
mod utils;

use crate::config::AppConfig;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "8080")]
    port: u16,

    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<database::Database>,
    pub cache: Arc<cache::CacheManager>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&args.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting compile-benchmark application");

    let config = Arc::new(AppConfig::default());
    let db = Arc::new(database::Database::new().await?);
    let cache = Arc::new(cache::CacheManager::new());
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let state = AppState {
        config,
        db,
        cache,
        http_client,
    };

    let app = create_router(state);

    let addr: SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/users", get(handlers::users::list_users))
        .route("/users", post(handlers::users::create_user))
        .route("/users/:id", get(handlers::users::get_user))
        .route("/users/:id", put(handlers::users::update_user))
        .route("/users/:id", delete(handlers::users::delete_user))
        .route("/posts", get(handlers::posts::list_posts))
        .route("/posts", post(handlers::posts::create_post))
        .route("/posts/:id", get(handlers::posts::get_post))
        .route("/posts/:id", put(handlers::posts::update_post))
        .route("/posts/:id", delete(handlers::posts::delete_post))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/refresh", post(handlers::auth::refresh_token))
        .route("/products", get(handlers::products::list_products))
        .route("/products/:id", get(handlers::products::get_product))
        .route("/orders", get(handlers::orders::list_orders))
        .route("/orders", post(handlers::orders::create_order))
        .route("/orders/:id", get(handlers::orders::get_order))
        .route("/analytics", get(handlers::analytics::get_analytics))
        .route("/search", get(handlers::search::search))
        .route("/upload", post(handlers::upload::upload_file))
        .route("/export", get(handlers::export::export_data));

    Router::new()
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
