use leptos::prelude::*;

use crate::{api::dto::TodoDto, components::todo_item::TodoItem};

#[component]
pub fn TodoList(
    todos: Resource<Result<Vec<TodoDto>, String>>,
) -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <section class="main"></section> }>
            {move || {
                let todos = todos.clone();

                Suspend::new(async move {
                    let todos = todos.await;

                    view! {
                        <section class="main">
                            <ul class="todo-list">
                                {match todos {
                                    Ok(items) => items
                                        .into_iter()
                                        .map(|todo| view! { <TodoItem todo /> })
                                        .collect_view()
                                        .into_any(),
                                    Err(error) => view! {
                                        <li class="todo">
                                            <div class="view">
                                                <label>{error}</label>
                                            </div>
                                        </li>
                                    }
                                    .into_any(),
                                }}
                            </ul>
                        </section>
                    }
                })
            }}
        </Suspense>
    }
}
