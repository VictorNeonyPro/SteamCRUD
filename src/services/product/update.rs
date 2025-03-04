use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateProductRequest {
    name: Option<String>,
    owner: Option<String>,
    price: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UpdateProductResponse {
    Success,
    UnknownProduct,
    AlreadyUsedName
}

pub async fn update_product(State(pool): State<PgPool>, Json(request): Json<UpdateProductRequest>) -> Json<UpdateProductResponse> {
    todo!()
}