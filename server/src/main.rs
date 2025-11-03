use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use std::{env, net::SocketAddr};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    dotenv();

    let _ = fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(cors);

    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(4000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn dotenv() {
    // load .env if present (optional dependency-less)
    if let Ok(path) = env::var("ENV_FILE") {
        let _ = std::fs::read_to_string(path);
    }
}

