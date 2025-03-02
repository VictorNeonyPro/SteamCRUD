use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::product::Product;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GetProductResponse {
    Success {
        name: String,
        creator: i32,
        price: BigDecimal
    },
    UnknownProduct,
}

pub async fn get_product(State(pool): State<PgPool>, Json(request): Json<GetProductRequest>) -> Json<GetProductResponse> {
    let result = sqlx::query_as!(Product, "SELECT * FROM Steam.Products WHERE name = $1;", request.name)
        .fetch_one(&pool)
        .await;
    
    match result {
        Ok(result) => Json(GetProductResponse::Success{ name: result.name, creator: result.creator, price: result.price }),
        Err(_) => Json(GetProductResponse::UnknownProduct),
    }
}

