use leptos::prelude::*;

pub mod components {
    pub mod footer;
    pub mod header;
    pub mod todo_item;
    pub mod todo_list;
}

pub mod api;

pub mod server_fns;

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
