mod crud;
mod models;

use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, patch},
    Router,
    Json
};
use axum::http::Method;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

use tower::limit::ConcurrencyLimitLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer};

use tower_http::trace::TraceLayer;

use sqlx::{PgPool, postgres::PgPoolOptions, Postgres, Pool};

use tokio::net::TcpListener;
use tower::ServiceBuilder;
use crate::crud::steamaccount::*;
use crate::crud::product::*;
use crate::crud::transaction::*;

#[tokio::main]
async fn main() {
    // load enviroment variables
    dotenv::dotenv().expect("Failed to read .env file");

    // get environment variables
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
    let server_address = dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not found in .env file");

    // create database pool
    let database_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Failed to connect to database pool");

    // create tcp listener
    let tcp_listener = TcpListener::bind(server_address)
        .await.expect("Failed to bind to server address");

    println!("Listening on {}", tcp_listener.local_addr().unwrap());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // allow requests from any origin
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    // compose routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/steamaccounts", get(get_steamaccounts).post(create_steamaccount))
        .route("/steamaccounts/:steamaccount_id", patch(update_steamaccount).delete(delete_steamaccount))
        .route("/products", get(get_products).post(create_product))
        .route("/products/:product_id", patch(update_product).delete(delete_product))
        .route("/transactions", get(get_transactions).post(create_transaction))
        .route("/transactions/:product_id", patch(update_transaction).delete(delete_transaction))
        .with_state(database_pool)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        );

    // serve application
    axum::serve(tcp_listener, app)
        .await.expect("Failed to run server");
}


