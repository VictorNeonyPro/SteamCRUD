use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub creator: i32,
    pub price: BigDecimal,
}

#[derive(Serialize, Deserialize)]
pub enum CreateProductResponse {
    Ok,
    PreexistingProduct,
}

pub async fn create_product(State(pool): State<PgPool>, Json(product): Json<CreateProductRequest>) -> Json<CreateProductResponse> {
    let result = sqlx::query!("INSERT INTO Steam.Products (name, creator, price) VALUES ($1, $2, $3)", product.name, product.creator, product.price)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(CreateProductResponse::Ok),
        Err(_) => Json(CreateProductResponse::PreexistingProduct),
    }
}