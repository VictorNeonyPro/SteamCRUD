mod crud;
mod models;

use axum::http::Method;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router
};
use serde::{Deserialize, Serialize};

use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

use tower_http::trace::TraceLayer;

use sqlx::postgres::PgPoolOptions;

use crate::crud::product::create::create_product;
use crate::crud::product::{delete_product, get_product, update_product};
use tokio::net::TcpListener;
use tower::ServiceBuilder;

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
        .route("/products", get(get_product).post(create_product).patch(update_product).delete(delete_product))
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


