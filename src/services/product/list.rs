use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
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

pub async fn list_products(State(pool): State<PgPool>, Json(request): Json<ListProductRequest>) -> Json<ListProductResponse> {
    let products = sqlx::query_as!(Product, "SELECT * FROM Steam.Products WHERE id BETWEEN $1 AND $2", request.from, request.to)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    
    Json(ListProductResponse { products })
}


