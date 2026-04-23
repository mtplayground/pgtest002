#[cfg(feature = "ssr")]
mod app;

#[cfg(feature = "ssr")]
use axum::Router;

#[cfg(feature = "ssr")]
use leptos::config::get_configuration;

#[cfg(feature = "ssr")]
use leptos_axum::{LeptosRoutes, file_and_error_handler, generate_route_list};

#[cfg(feature = "ssr")]
use crate::app::{App, shell};

#[cfg(feature = "ssr")]
fn config_error(error: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(format!("failed to load Leptos configuration: {error}"))
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conf = get_configuration(Some("Cargo.toml")).map_err(config_error)?;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(
            &leptos_options,
            routes,
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options.clone());

    let listener = tokio::net::TcpListener::bind(leptos_options.site_addr).await?;

    tracing::info!("listening on http://{}", leptos_options.site_addr);

    axum::serve(listener, app.into_make_service()).await
}

#[cfg(not(feature = "ssr"))]
fn main() {}
