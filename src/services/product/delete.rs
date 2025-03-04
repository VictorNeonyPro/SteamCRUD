use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Copy, Clone, Serialize ,Deserialize)]
pub enum DeleteProductResponse {
    Success,
    NotFound,
}

pub async fn delete_product(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<DeleteProductResponse> {
    let result = sqlx::query!("DELETE FROM Steam.Products WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(DeleteProductResponse::Success),
        Err(_) => Json(DeleteProductResponse::NotFound)
    }
}