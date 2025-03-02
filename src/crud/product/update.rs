use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UpdateProductRequest {
    ChangeName { name: String, new_name: String },
    ChangeCreator { name: String, new_creator: i32 },
    ChangePrice { name: String, new_price: BigDecimal },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UpdateProductResponse {
    Success,
    UnknownProduct,
    AlreadyUsedName
}

pub async fn update_product(State(pool): State<PgPool>, Json(request): Json<UpdateProductRequest>) -> Json<UpdateProductResponse> {
    let result = match request {
        UpdateProductRequest::ChangeName { name, new_name } => sqlx::query!("UPDATE Steam.Products SET name = $1 WHERE name = $2", new_name, name),
        UpdateProductRequest::ChangeCreator { name, new_creator } => sqlx::query!("UPDATE Steam.Products SET creator = $1 WHERE name = $2", new_creator, name),
        UpdateProductRequest::ChangePrice { name, new_price } => sqlx::query!("UPDATE Steam.Products SET price = $1 WHERE name = $2", new_price, name),
    }.execute(&pool).await;

    match result {
        Ok(_) => Json(UpdateProductResponse::Success),
        Err(error) => match error.as_database_error() {
            Some(error) if error.is_unique_violation() => Json(UpdateProductResponse::AlreadyUsedName),
            _ => Json(UpdateProductResponse::UnknownProduct)
        }
    }
}