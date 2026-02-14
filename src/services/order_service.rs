use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    cache::{cache_key, CacheManager},
    database::Database,
    error::Result,
    models::{
        Address, CreateOrderRequest, FulfillmentStatus, Order, OrderItemResponse, OrderResponse,
        OrderStatus, PaginationParams, PaymentStatus,
    },
};

pub struct OrderService {
    db: Arc<Database>,
    cache: Arc<CacheManager>,
}

impl OrderService {
    pub fn new(db: Arc<Database>, cache: Arc<CacheManager>) -> Self {
        Self { db, cache }
    }

    pub async fn list_orders(&self, pagination: &PaginationParams) -> Result<(Vec<OrderResponse>, i64)> {
        let _offset = (pagination.page - 1) * pagination.per_page;
        let _ = &self.db.pool;

        let orders: Vec<OrderResponse> = Vec::new();
        let total: i64 = 0;

        Ok((orders, total))
    }

    pub async fn get_order_by_id(&self, id: Uuid) -> Result<Option<OrderResponse>> {
        let cache_key = cache_key("order", &[&id.to_string()]);

        if let Some(order) = self.cache.get_json::<OrderResponse>(&cache_key).await {
            return Ok(Some(order));
        }

        let order: Option<OrderResponse> = None;
        Ok(order)
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<OrderResponse> {
        let now = Utc::now();
        let order_id = Uuid::new_v4();
        let order_number = generate_order_number();

        let billing_address = Address {
            first_name: request.billing_address.first_name,
            last_name: request.billing_address.last_name,
            company: request.billing_address.company,
            address_line_1: request.billing_address.address_line_1,
            address_line_2: request.billing_address.address_line_2,
            city: request.billing_address.city,
            state: request.billing_address.state,
            postal_code: request.billing_address.postal_code,
            country: request.billing_address.country,
            phone: request.billing_address.phone,
        };

        let shipping_address = Address {
            first_name: request.shipping_address.first_name,
            last_name: request.shipping_address.last_name,
            company: request.shipping_address.company,
            address_line_1: request.shipping_address.address_line_1,
            address_line_2: request.shipping_address.address_line_2,
            city: request.shipping_address.city,
            state: request.shipping_address.state,
            postal_code: request.shipping_address.postal_code,
            country: request.shipping_address.country,
            phone: request.shipping_address.phone,
        };

        let items: Vec<OrderItemResponse> = request
            .items
            .iter()
            .map(|item| OrderItemResponse {
                id: Uuid::new_v4(),
                product_id: item.product_id,
                sku: "SKU-XXX".to_string(),
                name: "Product Name".to_string(),
                quantity: item.quantity,
                unit_price: 29.99,
                total_price: 29.99 * item.quantity as f64,
            })
            .collect();

        let subtotal: f64 = items.iter().map(|i| i.total_price).sum();
        let tax_amount = subtotal * 0.1;
        let shipping_amount = 9.99;

        let response = OrderResponse {
            id: order_id,
            order_number,
            customer_id: request.customer_id,
            status: OrderStatus::Pending,
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            items,
            subtotal,
            tax_amount,
            shipping_amount,
            discount_amount: 0.0,
            total: subtotal + tax_amount + shipping_amount,
            currency: "USD".to_string(),
            billing_address,
            shipping_address,
            tracking_number: None,
            placed_at: now,
        };

        let cache_key = cache_key("order", &[&order_id.to_string()]);
        let _ = self.cache.set_json(cache_key, &response).await;

        Ok(response)
    }

    pub async fn update_order_status(&self, id: Uuid, status: OrderStatus) -> Result<()> {
        let _ = &self.db.pool;
        let _ = id;
        let _ = status;

        let cache_key = cache_key("order", &[&id.to_string()]);
        self.cache.delete(&cache_key).await;

        Ok(())
    }

    pub async fn get_orders_by_customer(&self, customer_id: Uuid) -> Result<Vec<OrderResponse>> {
        let _ = &self.db.pool;
        let _ = customer_id;
        Ok(Vec::new())
    }

    pub async fn cancel_order(&self, id: Uuid) -> Result<()> {
        self.update_order_status(id, OrderStatus::Cancelled).await
    }
}

fn generate_order_number() -> String {
    let now = Utc::now();
    format!(
        "ORD-{}-{}",
        now.format("%Y%m%d"),
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    )
}
