use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteProductRequest {
    pub name: String
}

#[derive(Copy, Clone, Serialize ,Deserialize)]
pub enum DeleteProductResponse {
    Success,
    NotFound,
}

pub async fn delete_product(State(pool): State<PgPool>, Json(request): Json<DeleteProductRequest>) -> Json<DeleteProductResponse> {
    let result = sqlx::query!("DELETE FROM Steam.Products WHERE name = $1", request.name)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(DeleteProductResponse::Success),
        Err(_) => Json(DeleteProductResponse::NotFound)
    }
}