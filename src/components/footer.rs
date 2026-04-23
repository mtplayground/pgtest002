use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::server_fns::ClearCompleted;

#[component]
pub fn Footer(
    #[prop(into)] active_count: Signal<usize>,
    #[prop(into)] has_completed: Signal<bool>,
    #[prop(optional, into)] invalidate_todos: Option<Callback<()>>,
) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    let clear_completed_action = ServerAction::<ClearCompleted>::new();
    let clear_completed_value = clear_completed_action.value();
    let invalidate_todos = StoredValue::new(invalidate_todos);

    Effect::new(move |_| {
        if matches!(clear_completed_value.get(), Some(Ok(()))) {
            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{move || active_count.get()}</strong>
                <span>
                    {move || {
                        let count = active_count.get();
                        format!(" item{} left", if count == 1 { "" } else { "s" })
                    }}
                </span>
            </span>

            <ul class="filters">
                <li>
                    <a class=move || filter_class(&pathname(), "/") href="/">
                        "All"
                    </a>
                </li>
                <li>
                    <a class=move || filter_class(&pathname(), "/active") href="/active">
                        "Active"
                    </a>
                </li>
                <li>
                    <a class=move || filter_class(&pathname(), "/completed") href="/completed">
                        "Completed"
                    </a>
                </li>
            </ul>

            <button
                class="clear-completed"
                disabled=move || clear_completed_action.pending().get()
                hidden=move || !has_completed.get()
                on:click=move |_| {
                    clear_completed_action.dispatch(ClearCompleted {});
                }
                type="button"
            >
                "Clear completed"
            </button>
        </footer>
    }
}

fn filter_class(current_path: &str, target_path: &str) -> &'static str {
    if current_path == target_path {
        "selected"
    } else {
        ""
    }
}
