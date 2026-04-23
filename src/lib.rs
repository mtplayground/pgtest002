use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod api;

#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "ssr")]
pub mod todos {
    pub mod model;
    pub mod repo;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"pgtest002"</h1>
        </main>
    }
}
