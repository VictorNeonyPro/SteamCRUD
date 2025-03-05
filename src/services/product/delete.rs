use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub async fn delete_product(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    let result = sqlx::query!("DELETE FROM Steam.Products WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}