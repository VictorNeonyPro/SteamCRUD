use std::any::Any;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use crate::models::product::Product;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UpdateProductRequest {
    ChangeName { name: String, new_name: String },
    ChangeOwner { name: String, new_owner: String }
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
        UpdateProductRequest::ChangeOwner { name, new_owner } => sqlx::query!("UPDATE Steam.Products SET owner = $1 WHERE name = $2", new_owner, name)
    }.execute(&pool).await;
    
    match result {
        Ok(_) => Json(UpdateProductResponse::Success),
        Err(error) => match error.as_database_error() {
            Some(error) if error.is_unique_violation() => Json(UpdateProductResponse::AlreadyUsedName),
            _ => Json(UpdateProductResponse::UnknownProduct)
        }
    }
}