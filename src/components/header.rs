use leptos::{ev::SubmitEvent, prelude::*};

use crate::server_fns::CreateTodo;

#[component]
pub fn Header(
    #[prop(optional, into)] invalidate_todos: Option<Callback<()>>,
) -> impl IntoView {
    let (new_todo, set_new_todo) = signal(String::new());
    let create_todo_action = ServerAction::<CreateTodo>::new();
    let action_value = create_todo_action.value();
    let invalidate_todos = StoredValue::new(invalidate_todos);

    Effect::new(move |_| {
        if matches!(action_value.get(), Some(Ok(_))) {
            set_new_todo.set(String::new());

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
            <form on:submit=on_submit>
                <label class="visually-hidden" for="new-todo-input">
                    "New todo"
                </label>
                <input
                    id="new-todo-input"
                    aria-label="New todo"
                    autofocus
                    class="new-todo"
                    disabled=move || create_todo_action.pending().get()
                    on:input=move |event| set_new_todo.set(event_target_value(&event))
                    placeholder="What needs to be done?"
                    prop:value=move || new_todo.get()
                    type="text"
                />
            </form>
        </header>
    }
}
