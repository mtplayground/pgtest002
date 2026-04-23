#[cfg(feature = "ssr")]
mod app;
#[cfg(feature = "ssr")]
mod config;

#[cfg(feature = "ssr")]
use axum::Router;

#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;

#[cfg(feature = "ssr")]
use leptos_axum::{LeptosRoutes, file_and_error_handler, generate_route_list};

#[cfg(feature = "ssr")]
use crate::app::{App, shell};

#[cfg(feature = "ssr")]
use crate::config::Config;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config =
        Config::from_env().map_err(|error| std::io::Error::other(error.to_string()))?;
    let leptos_options: LeptosOptions = axum::extract::FromRef::from_ref(&config);
    let routes = generate_route_list(App);

    let app = Router::<Config>::new()
        .leptos_routes(
            &config,
            routes,
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(file_and_error_handler::<Config, _>(shell))
        .with_state(config.clone());

    let listener = tokio::net::TcpListener::bind(config.listen_addr()).await?;

    tracing::info!(
        "listening on http://{} with site root {}",
        config.leptos_site_addr,
        config.leptos_site_root
    );

    axum::serve(listener, app.into_make_service()).await
}

#[cfg(not(feature = "ssr"))]
fn main() {}
