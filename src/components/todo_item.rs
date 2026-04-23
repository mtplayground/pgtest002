use leptos::prelude::*;

use crate::api::dto::TodoDto;

#[component]
pub fn TodoItem(todo: TodoDto) -> impl IntoView {
    let item_class = if todo.completed {
        "todo completed"
    } else {
        "todo"
    };
    let destroy_label = format!("Delete todo {}", todo.title);

    view! {
        <li class=item_class>
            <div class="view">
                <input
                    checked=todo.completed
                    class="toggle"
                    disabled
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
