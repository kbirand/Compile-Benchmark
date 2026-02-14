use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TypedBuilder)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub short_description: Option<String>,
    pub price: f64,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub currency: String,
    pub quantity: i32,
    pub low_stock_threshold: i32,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub images: Vec<String>,
    pub thumbnail_url: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub status: ProductStatus,
    pub is_featured: bool,
    pub is_digital: bool,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub struct ProductDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "product_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
    OutOfStock,
    Discontinued,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TypedBuilder)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub short_description: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    #[validate(length(equal = 3))]
    pub currency: Option<String>,
    #[validate(range(min = 0))]
    pub quantity: i32,
    pub low_stock_threshold: Option<i32>,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub images: Option<Vec<String>>,
    pub category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub is_featured: Option<bool>,
    pub is_digital: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub short_description: Option<String>,
    pub price: f64,
    pub sale_price: Option<f64>,
    pub currency: String,
    pub quantity: i32,
    pub in_stock: bool,
    pub images: Vec<String>,
    pub thumbnail_url: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub status: ProductStatus,
    pub is_featured: bool,
    pub is_digital: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            sku: product.sku,
            name: product.name,
            slug: product.slug,
            description: product.description,
            short_description: product.short_description,
            price: product.price,
            sale_price: product.sale_price,
            currency: product.currency,
            quantity: product.quantity,
            in_stock: product.quantity > 0,
            images: product.images,
            thumbnail_url: product.thumbnail_url,
            category_id: product.category_id,
            brand_id: product.brand_id,
            status: product.status,
            is_featured: product.is_featured,
            is_digital: product.is_digital,
            created_at: product.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub products: Vec<ProductResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
