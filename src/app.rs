use leptos::config::LeptosOptions;
use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_meta::{Stylesheet, provide_meta_context};
use leptos_router::components::Router;

#[path = "routes.rs"]
mod routes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (filter, set_filter) = signal(routes::TodoFilter::All);
    routes::provide_todo_filter_signal(filter);

    view! {
        <Router>
            <main class="todoapp" data-filter=move || filter.get().as_str()>
                <header class="header">
                    <h1>"todos"</h1>
                    <input
                        aria-label="Create a new todo"
                        autofocus
                        class="new-todo"
                        placeholder="What needs to be done?"
                        type="text"
                    />
                </header>
                <routes::FilterRoutes set_filter />
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta content="width=device-width, initial-scale=1" name="viewport" />
                <meta content="A TodoMVC shell rendered by Leptos and served by Axum." name="description" />
                <Stylesheet href="/assets/todomvc.css" id="todomvc-css" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <title>"pgtest002"</title>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
