mod services;
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

use crate::services::product::{create::create_product, delete::delete_product, get::get_product, update::update_product};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use crate::services::product::list::list_products;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
    let server_address = dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not found in .env file");
    
    let database_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Failed to connect to database pool");
    
    let tcp_listener = TcpListener::bind(server_address)
        .await.expect("Failed to bind to server address");

    println!("Listening on {}", tcp_listener.local_addr().unwrap());

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());
    
    let app = Router::new()
        .route("/products", get(list_products).post(create_product))
        .route("/products/:id", get(get_product).delete(delete_product).patch(update_product))
        .with_state(database_pool)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        );
    
    axum::serve(tcp_listener, app)
        .await.expect("Failed to run server");
}


