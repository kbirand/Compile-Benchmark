use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AnalyticsOverview {
    pub total_users: i64,
    pub active_users: i64,
    pub new_users_today: i64,
    pub new_users_this_week: i64,
    pub new_users_this_month: i64,
    pub total_orders: i64,
    pub orders_today: i64,
    pub orders_this_week: i64,
    pub orders_this_month: i64,
    pub total_revenue: f64,
    pub revenue_today: f64,
    pub revenue_this_week: f64,
    pub revenue_this_month: f64,
    pub average_order_value: f64,
    pub conversion_rate: f64,
    pub total_products: i64,
    pub active_products: i64,
    pub out_of_stock_products: i64,
    pub total_page_views: i64,
    pub unique_visitors: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData {
    pub date: NaiveDate,
    pub value: f64,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub total_sold: i64,
    pub total_revenue: f64,
    pub average_rating: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCategory {
    pub category_id: Uuid,
    pub category_name: String,
    pub product_count: i64,
    pub total_sold: i64,
    pub total_revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub segment_name: String,
    pub customer_count: i64,
    pub percentage: f64,
    pub total_revenue: f64,
    pub average_order_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSource {
    pub source: String,
    pub medium: String,
    pub visits: i64,
    pub unique_visitors: i64,
    pub bounce_rate: f64,
    pub conversion_rate: f64,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicData {
    pub country: String,
    pub country_code: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub visitors: i64,
    pub orders: i64,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAnalytics {
    pub device_type: String,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub visits: i64,
    pub percentage: f64,
    pub conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub granularity: Option<Granularity>,
    pub metrics: Option<Vec<String>>,
    pub dimensions: Option<Vec<String>>,
    pub filters: Option<Vec<AnalyticsFilter>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Granularity {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    Contains,
    In,
    NotIn,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResponse {
    pub overview: AnalyticsOverview,
    pub revenue_chart: ChartData,
    pub orders_chart: ChartData,
    pub visitors_chart: ChartData,
    pub top_products: Vec<TopProduct>,
    pub top_categories: Vec<TopCategory>,
    pub customer_segments: Vec<CustomerSegment>,
    pub traffic_sources: Vec<TrafficSource>,
    pub geographic_data: Vec<GeographicData>,
    pub device_analytics: Vec<DeviceAnalytics>,
}
