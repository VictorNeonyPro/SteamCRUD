use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::product::Product;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductRequest {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductResponse {
    product: Option<Product>,
}

pub async fn get_product(State(pool): State<PgPool>, Path(id): Path<i32>) -> (StatusCode, Json<GetProductResponse>) {
    let result = sqlx::query_as!(Product, "SELECT * FROM Steam.Products WHERE id = $1;", id)
        .fetch_one(&pool)
        .await;
    
    match result {
        Ok(product) => (StatusCode::OK, Json(GetProductResponse { product: Some(product) })),
        Err(_) => (StatusCode::NOT_FOUND, Json(GetProductResponse { product: None })),
    }
}

