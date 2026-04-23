#[cfg(feature = "ssr")]
use axum::{Router, response::IntoResponse, routing::get};

#[cfg(feature = "ssr")]
async fn healthcheck() -> impl IntoResponse {
    "ok"
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new().route("/health", get(healthcheck));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("listening on http://0.0.0.0:8080");

    axum::serve(listener, app).await
}

#[cfg(not(feature = "ssr"))]
fn main() {}
