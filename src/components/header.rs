use leptos::{ev::SubmitEvent, prelude::*};

use crate::server_fns::{CreateTodo, ToggleAll};

#[component]
pub fn Header(
    #[prop(optional)] all_completed: Option<Signal<bool>>,
    #[prop(optional)] has_todos: Option<Signal<bool>>,
    #[prop(optional, into)] invalidate_todos: Option<Callback<()>>,
) -> impl IntoView {
    let (new_todo, set_new_todo) = signal(String::new());
    let create_todo_action = ServerAction::<CreateTodo>::new();
    let create_todo_value = create_todo_action.value();
    let toggle_all_action = ServerAction::<ToggleAll>::new();
    let toggle_all_value = toggle_all_action.value();
    let all_completed = StoredValue::new(all_completed);
    let has_todos = StoredValue::new(has_todos);
    let invalidate_todos = StoredValue::new(invalidate_todos);

    Effect::new(move |_| {
        if matches!(create_todo_value.get(), Some(Ok(_))) {
            set_new_todo.set(String::new());

            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    Effect::new(move |_| {
        if matches!(toggle_all_value.get(), Some(Ok(()))) {
            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let title = new_todo.get_untracked();

        if title.trim().is_empty() {
            return;
        }

        create_todo_action.dispatch(CreateTodo { title });
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                checked=move || {
                    all_completed
                        .get_value()
                        .as_ref()
                        .map(|signal| signal.get())
                        .unwrap_or(false)
                }
                class="toggle-all"
                disabled=move || toggle_all_action.pending().get()
                hidden=move || {
                    !has_todos
                        .get_value()
                        .as_ref()
                        .map(|signal| signal.get())
                        .unwrap_or(false)
                }
                id="toggle-all"
                on:change=move |event| {
                    toggle_all_action.dispatch(ToggleAll {
                        completed: event_target_checked(&event),
                    });
                }
                type="checkbox"
            />
            <label for="toggle-all" hidden=move || {
                !has_todos
                    .get_value()
                    .as_ref()
                    .map(|signal| signal.get())
                    .unwrap_or(false)
            }>
                "Mark all as complete"
            </label>
            <form on:submit=on_submit>
                <label class="visually-hidden" for="new-todo-input">
                    "New todo"
                </label>
                <input
                    id="new-todo-input"
                    aria-label="New todo"
                    autofocus
                    class="new-todo"
                    disabled=move || {
                        create_todo_action.pending().get() || toggle_all_action.pending().get()
                    }
                    on:input=move |event| set_new_todo.set(event_target_value(&event))
                    placeholder="What needs to be done?"
                    prop:value=move || new_todo.get()
                    type="text"
                />
            </form>
        </header>
    }
}
