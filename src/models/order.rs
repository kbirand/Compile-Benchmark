use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TypedBuilder)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub shipping_amount: f64,
    pub discount_amount: f64,
    pub total: f64,
    pub currency: String,
    pub billing_address: Address,
    pub shipping_address: Address,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub placed_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Paid,
    PartiallyPaid,
    Refunded,
    PartiallyRefunded,
    Failed,
    Voided,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "fulfillment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentStatus {
    Unfulfilled,
    PartiallyFulfilled,
    Fulfilled,
    Shipped,
    Delivered,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TypedBuilder)]
pub struct CreateOrderRequest {
    pub customer_id: Uuid,
    #[validate(length(min = 1))]
    pub items: Vec<CreateOrderItemRequest>,
    #[validate]
    pub billing_address: AddressRequest,
    #[validate]
    pub shipping_address: AddressRequest,
    pub shipping_method: Option<String>,
    pub notes: Option<String>,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddressRequest {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    pub company: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub city: String,
    pub state: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub postal_code: String,
    #[validate(length(equal = 2))]
    pub country: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub items: Vec<OrderItemResponse>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub shipping_amount: f64,
    pub discount_amount: f64,
    pub total: f64,
    pub currency: String,
    pub billing_address: Address,
    pub shipping_address: Address,
    pub tracking_number: Option<String>,
    pub placed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub orders: Vec<OrderResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}
