use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::product::Product;

#[derive(Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub enum CreateProductResponse {
    Ok,
    PreexistingProduct,
}

pub async fn create_product(State(pool): State<PgPool>, Json(product): Json<CreateProductRequest>) -> Json<CreateProductResponse> {
    let result = sqlx::query!("INSERT INTO Steam.Products (name, owner) VALUES ($1, $2)", product.name, product.owner)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(CreateProductResponse::Ok),
        Err(_) => Json(CreateProductResponse::PreexistingProduct),
    }
}