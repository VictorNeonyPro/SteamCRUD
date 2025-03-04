use axum::Json;
use axum::{extract::State, http::StatusCode};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateProductRequest {
    name: Option<String>,
    owner: Option<String>,
    price: Option<BigDecimal>,
}

pub async fn update_product(
    State(pool): State<PgPool>,
    Json(request): Json<UpdateProductRequest>,
) -> StatusCode {
    let elements = [
        request.name.map(|name| format!("name = {}", name)),
        request.owner.map(|owner| format!("owner = {}", owner)),
        request.price.map(|price| format!("price = {}", price)),
    ];

    let elements: Vec<String> = elements.iter().filter_map(|e| e.clone()).collect();
    let query = format!("UPDATE Steam.Products SET {}", elements.join(","));
    let query = sqlx::query(&query).execute(&pool).await;
    if let Err(_) = query {
        StatusCode::NOT_MODIFIED
    } else {
        StatusCode::OK
    }
}
