#[cfg(feature = "ssr")]
mod app;
#[cfg(feature = "ssr")]
mod config;
#[cfg(feature = "ssr")]
mod db;
#[cfg(feature = "ssr")]
mod state;

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
use crate::db::create_pool;
#[cfg(feature = "ssr")]
use crate::state::AppState;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config =
        Config::from_env().map_err(|error| std::io::Error::other(error.to_string()))?;
    let leptos_options: LeptosOptions = axum::extract::FromRef::from_ref(&config);
    let pool = create_pool(&config.database_url)
        .await
        .map_err(|error| std::io::Error::other(format!("failed to create Postgres pool: {error}")))?;
    let state = AppState::new(pool, leptos_options.clone());
    let routes = generate_route_list(App);

    let app = Router::<AppState>::new()
        .leptos_routes(
            &state,
            routes,
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

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
