use std::time::Instant;

use axum::{
    body::Body,
    extract::MatchedPath,
    http::Request,
    middleware::Next,
    response::Response,
};
use tracing::Instrument;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() -> Result<(), tracing_subscriber::util::TryInitError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("pgtest002=info,tower_http=info,sqlx=warn,axum=info")
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(false).compact())
        .try_init()
}

pub async fn request_span(request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .map(str::to_owned);
    let path = matched_path.unwrap_or_else(|| request.uri().path().to_string());
    let span = tracing::info_span!("http_request", method = %method, path = %path);

    async move {
        let started_at = Instant::now();
        let response = next.run(request).await;
        let status = response.status();
        let elapsed_ms = started_at.elapsed().as_millis();

        if status.is_server_error() {
            tracing::error!(status = %status.as_u16(), elapsed_ms, "request completed with server error");
        } else {
            tracing::info!(status = %status.as_u16(), elapsed_ms, "request completed");
        }

        response
    }
    .instrument(span)
    .await
}
