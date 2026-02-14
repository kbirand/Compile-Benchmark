use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{error::Result, AppState};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub sort_by: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub result_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub price: Option<f64>,
    pub rating: Option<f64>,
    pub relevance_score: f64,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub facets: SearchFacets,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchFacets {
    pub categories: Vec<FacetItem>,
    pub price_ranges: Vec<FacetItem>,
    pub ratings: Vec<FacetItem>,
}

#[derive(Debug, Serialize)]
pub struct FacetItem {
    pub value: String,
    pub count: i64,
}

pub async fn search(
    State(_state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<SearchQuery>,
) -> Result<Json<SearchResponse>> {
    // Simulated search results
    let results = vec![
        SearchResult {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Result for: {}", query.q),
            description: "This is a sample search result description.".to_string(),
            result_type: "product".to_string(),
            url: "/products/sample-product".to_string(),
            thumbnail_url: Some("/images/sample.jpg".to_string()),
            price: Some(29.99),
            rating: Some(4.5),
            relevance_score: 0.95,
        },
        SearchResult {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Another result for: {}", query.q),
            description: "Another sample search result.".to_string(),
            result_type: "post".to_string(),
            url: "/posts/sample-post".to_string(),
            thumbnail_url: None,
            price: None,
            rating: None,
            relevance_score: 0.85,
        },
    ];

    let facets = SearchFacets {
        categories: vec![
            FacetItem {
                value: "Electronics".to_string(),
                count: 150,
            },
            FacetItem {
                value: "Clothing".to_string(),
                count: 89,
            },
        ],
        price_ranges: vec![
            FacetItem {
                value: "$0 - $25".to_string(),
                count: 234,
            },
            FacetItem {
                value: "$25 - $50".to_string(),
                count: 156,
            },
        ],
        ratings: vec![
            FacetItem {
                value: "4+ Stars".to_string(),
                count: 312,
            },
            FacetItem {
                value: "3+ Stars".to_string(),
                count: 456,
            },
        ],
    };

    let response = SearchResponse {
        query: query.q,
        results,
        total: 2,
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
        facets,
        suggestions: vec![
            "suggestion 1".to_string(),
            "suggestion 2".to_string(),
        ],
    };

    Ok(Json(response))
}
