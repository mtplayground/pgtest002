use leptos::prelude::*;

use crate::{api::dto::TodoDto, server_fns::SetCompleted};

#[component]
pub fn TodoItem(
    todo: TodoDto,
    #[prop(optional, into)] invalidate_todos: Option<Callback<()>>,
) -> impl IntoView {
    let item_class = if todo.completed {
        "todo completed"
    } else {
        "todo"
    };
    let destroy_label = format!("Delete todo {}", todo.title);
    let set_completed_action = ServerAction::<SetCompleted>::new();
    let action_value = set_completed_action.value();
    let invalidate_todos = StoredValue::new(invalidate_todos);
    let id = todo.id;

    Effect::new(move |_| {
        if matches!(action_value.get(), Some(Ok(_))) {
            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    view! {
        <li class=item_class>
            <div class="view">
                <input
                    checked=todo.completed
                    class="toggle"
                    disabled=move || set_completed_action.pending().get()
                    on:change=move |event| {
                        set_completed_action.dispatch(SetCompleted {
                            id,
                            completed: event_target_checked(&event),
                        });
                    }
                    type="checkbox"
                />
                <label>{todo.title}</label>
                <button
                    aria-label=destroy_label
                    class="destroy"
                    disabled
                    type="button"
                ></button>
            </div>
        </li>
    }
}
