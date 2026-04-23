use leptos::{html, prelude::*};

use crate::{
    api::dto::TodoDto,
    server_fns::{DeleteTodo, SetCompleted, UpdateTitle},
};

#[component]
pub fn TodoItem(
    todo: TodoDto,
    #[prop(optional, into)] invalidate_todos: Option<Callback<()>>,
) -> impl IntoView {
    let (is_editing, set_is_editing) = signal(false);
    let (draft_title, set_draft_title) = signal(todo.title.clone());
    let original_title = StoredValue::new(todo.title.clone());
    let destroy_label = format!("Delete todo {}", todo.title);
    let set_completed_action = ServerAction::<SetCompleted>::new();
    let set_completed_value = set_completed_action.value();
    let update_title_action = ServerAction::<UpdateTitle>::new();
    let update_title_value = update_title_action.value();
    let delete_todo_action = ServerAction::<DeleteTodo>::new();
    let delete_todo_value = delete_todo_action.value();
    let invalidate_todos = StoredValue::new(invalidate_todos);
    let id = todo.id;
    let edit_input_ref = NodeRef::<html::Input>::new();

    Effect::new(move |_| {
        if matches!(set_completed_value.get(), Some(Ok(_))) {
            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    Effect::new(move |_| {
        if is_editing.get() {
            if let Some(input) = edit_input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(updated_todo)) = update_title_value.get() {
            set_is_editing.set(false);

            if let Some(todo) = updated_todo {
                set_draft_title.set(todo.title);
            }

            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    Effect::new(move |_| {
        if matches!(delete_todo_value.get(), Some(Ok(()))) {
            set_is_editing.set(false);

            if let Some(callback) = invalidate_todos.get_value() {
                callback.run(());
            }
        }
    });

    let save_edit = move || {
        update_title_action.dispatch(UpdateTitle {
            id,
            title: draft_title.get_untracked(),
        });
    };

    let cancel_edit = move || {
        set_draft_title.set(original_title.get_value());
        set_is_editing.set(false);
    };

    view! {
        <li class=move || {
            let mut classes = vec!["todo"];

            if todo.completed {
                classes.push("completed");
            }

            if is_editing.get() {
                classes.push("editing");
            }

            classes.join(" ")
        }>
            <div class="view">
                <input
                    checked=todo.completed
                    class="toggle"
                    disabled=move || {
                        set_completed_action.pending().get()
                            || update_title_action.pending().get()
                            || delete_todo_action.pending().get()
                    }
                    on:change=move |event| {
                        set_completed_action.dispatch(SetCompleted {
                            id,
                            completed: event_target_checked(&event),
                        });
                    }
                    type="checkbox"
                />
                <label on:dblclick=move |_| {
                    set_draft_title.set(original_title.get_value());
                    set_is_editing.set(true);
                }>{todo.title}</label>
                <button
                    aria-label=destroy_label
                    class="destroy"
                    disabled=move || {
                        set_completed_action.pending().get()
                            || update_title_action.pending().get()
                            || delete_todo_action.pending().get()
                    }
                    on:click=move |_| {
                        delete_todo_action.dispatch(DeleteTodo { id });
                    }
                    type="button"
                ></button>
            </div>
            <input
                class="edit"
                disabled=move || {
                    update_title_action.pending().get() || delete_todo_action.pending().get()
                }
                node_ref=edit_input_ref
                on:input=move |event| set_draft_title.set(event_target_value(&event))
                on:keydown=move |event| match event.key().as_str() {
                    "Enter" => {
                        event.prevent_default();
                        save_edit();
                    }
                    "Escape" => {
                        event.prevent_default();
                        cancel_edit();
                    }
                    _ => {}
                }
                prop:value=move || draft_title.get()
                type="text"
            />
        </li>
    }
}
