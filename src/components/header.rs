use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn Header() -> impl IntoView {
    let (new_todo, set_new_todo) = signal(String::new());

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let _ = new_todo.get();
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
                    on:input=move |event| set_new_todo.set(event_target_value(&event))
                    placeholder="What needs to be done?"
                    prop:value=move || new_todo.get()
                    type="text"
                />
            </form>
        </header>
    }
}
