use axum::{extract::State, Json};

use crate::{
    error::Result,
    models::{
        AnalyticsOverview, AnalyticsResponse, ChartData, CustomerSegment, Dataset,
        DeviceAnalytics, GeographicData, TopCategory, TopProduct, TrafficSource,
    },
    AppState,
};

pub async fn get_analytics(State(_state): State<AppState>) -> Result<Json<AnalyticsResponse>> {
    // Generate sample analytics data
    let overview = AnalyticsOverview::builder()
        .total_users(15420)
        .active_users(8234)
        .new_users_today(142)
        .new_users_this_week(876)
        .new_users_this_month(3421)
        .total_orders(42156)
        .orders_today(89)
        .orders_this_week(534)
        .orders_this_month(2341)
        .total_revenue(1542367.89)
        .revenue_today(12456.78)
        .revenue_this_week(78234.56)
        .revenue_this_month(342156.78)
        .average_order_value(36.58)
        .conversion_rate(3.24)
        .total_products(1234)
        .active_products(1089)
        .out_of_stock_products(45)
        .total_page_views(542367)
        .unique_visitors(123456)
        .build();

    let revenue_chart = ChartData {
        labels: vec![
            "Jan".to_string(),
            "Feb".to_string(),
            "Mar".to_string(),
            "Apr".to_string(),
            "May".to_string(),
            "Jun".to_string(),
        ],
        datasets: vec![Dataset {
            label: "Revenue".to_string(),
            data: vec![45000.0, 52000.0, 48000.0, 61000.0, 55000.0, 67000.0],
            background_color: Some("#4F46E5".to_string()),
            border_color: Some("#4F46E5".to_string()),
        }],
    };

    let orders_chart = ChartData {
        labels: vec![
            "Jan".to_string(),
            "Feb".to_string(),
            "Mar".to_string(),
            "Apr".to_string(),
            "May".to_string(),
            "Jun".to_string(),
        ],
        datasets: vec![Dataset {
            label: "Orders".to_string(),
            data: vec![1200.0, 1350.0, 1280.0, 1520.0, 1420.0, 1680.0],
            background_color: Some("#10B981".to_string()),
            border_color: Some("#10B981".to_string()),
        }],
    };

    let visitors_chart = ChartData {
        labels: vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        datasets: vec![Dataset {
            label: "Visitors".to_string(),
            data: vec![4200.0, 4500.0, 4800.0, 4300.0, 5200.0, 3800.0, 3200.0],
            background_color: Some("#F59E0B".to_string()),
            border_color: Some("#F59E0B".to_string()),
        }],
    };

    let top_products = vec![
        TopProduct {
            product_id: uuid::Uuid::new_v4(),
            product_name: "Premium Widget".to_string(),
            sku: "WDG-001".to_string(),
            total_sold: 1234,
            total_revenue: 61700.0,
            average_rating: Some(4.8),
        },
        TopProduct {
            product_id: uuid::Uuid::new_v4(),
            product_name: "Standard Gadget".to_string(),
            sku: "GDG-002".to_string(),
            total_sold: 987,
            total_revenue: 29610.0,
            average_rating: Some(4.5),
        },
    ];

    let top_categories = vec![TopCategory {
        category_id: uuid::Uuid::new_v4(),
        category_name: "Electronics".to_string(),
        product_count: 456,
        total_sold: 8765,
        total_revenue: 438250.0,
    }];

    let customer_segments = vec![
        CustomerSegment {
            segment_name: "VIP".to_string(),
            customer_count: 234,
            percentage: 1.5,
            total_revenue: 156000.0,
            average_order_value: 667.0,
        },
        CustomerSegment {
            segment_name: "Regular".to_string(),
            customer_count: 5678,
            percentage: 36.8,
            total_revenue: 425850.0,
            average_order_value: 75.0,
        },
    ];

    let traffic_sources = vec![
        TrafficSource {
            source: "google".to_string(),
            medium: "organic".to_string(),
            visits: 45678,
            unique_visitors: 34567,
            bounce_rate: 42.3,
            conversion_rate: 3.8,
            revenue: 234567.0,
        },
        TrafficSource {
            source: "facebook".to_string(),
            medium: "paid".to_string(),
            visits: 12345,
            unique_visitors: 10234,
            bounce_rate: 55.2,
            conversion_rate: 2.1,
            revenue: 67890.0,
        },
    ];

    let geographic_data = vec![
        GeographicData {
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: Some("California".to_string()),
            city: Some("Los Angeles".to_string()),
            visitors: 23456,
            orders: 1234,
            revenue: 98765.0,
        },
        GeographicData {
            country: "United Kingdom".to_string(),
            country_code: "GB".to_string(),
            region: Some("England".to_string()),
            city: Some("London".to_string()),
            visitors: 12345,
            orders: 567,
            revenue: 45678.0,
        },
    ];

    let device_analytics = vec![
        DeviceAnalytics {
            device_type: "Desktop".to_string(),
            browser: Some("Chrome".to_string()),
            os: Some("Windows".to_string()),
            visits: 56789,
            percentage: 55.2,
            conversion_rate: 4.1,
        },
        DeviceAnalytics {
            device_type: "Mobile".to_string(),
            browser: Some("Safari".to_string()),
            os: Some("iOS".to_string()),
            visits: 34567,
            percentage: 33.6,
            conversion_rate: 2.8,
        },
    ];

    let response = AnalyticsResponse {
        overview,
        revenue_chart,
        orders_chart,
        visitors_chart,
        top_products,
        top_categories,
        customer_segments,
        traffic_sources,
        geographic_data,
        device_analytics,
    };

    Ok(Json(response))
}
