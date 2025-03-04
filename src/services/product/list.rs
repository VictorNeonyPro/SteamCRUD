use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use crate::models::product::Product;

#[derive(Serialize, Deserialize)]
pub struct ListProductRequest {
    from: i32,
    to: i32
}

#[derive(Serialize, Deserialize)]
pub struct ListProductResponse {
    products: Vec<Product>
}

pub async fn list_products(State(pool): State<PgPool>, Json(request): Json<ListProductRequest>) -> (StatusCode, Json<ListProductResponse>) {
    let products = sqlx::query_as!(Product, "SELECT * FROM Steam.Products WHERE id BETWEEN $1 AND $2", request.from, request.to)
        .fetch_all(&pool)
        .await;
    
    if let Ok(products) = products {
        (StatusCode::OK, Json(ListProductResponse { products }))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ListProductResponse{ products: Vec::new() }))
    }
}


