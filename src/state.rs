use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub leptos_options: LeptosOptions,
}

impl AppState {
    pub fn new(pool: PgPool, leptos_options: LeptosOptions) -> Self {
        Self { pool, leptos_options }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
