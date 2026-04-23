use leptos::prelude::*;
use leptos_router::{components::{Route, Routes}, path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

impl TodoFilter {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Active => "active",
            Self::Completed => "completed",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Active => "Active",
            Self::Completed => "Completed",
        }
    }
}

#[derive(Clone, Copy)]
pub struct TodoFilterSignal(ReadSignal<TodoFilter>);

pub fn provide_todo_filter_signal(filter: ReadSignal<TodoFilter>) {
    provide_context(TodoFilterSignal(filter));
}

pub fn use_todo_filter_signal() -> ReadSignal<TodoFilter> {
    use_context::<TodoFilterSignal>()
        .map(|context| context.0)
        .expect("todo filter signal should be provided")
}

#[component]
pub fn FilterRoutes(set_filter: WriteSignal<TodoFilter>) -> impl IntoView {
    view! {
        <Routes fallback=move || view! { <FilterView filter=TodoFilter::All set_filter /> }>
            <Route path=path!("") view=move || view! { <FilterView filter=TodoFilter::All set_filter /> } />
            <Route path=path!("active") view=move || view! { <FilterView filter=TodoFilter::Active set_filter /> } />
            <Route path=path!("completed") view=move || view! { <FilterView filter=TodoFilter::Completed set_filter /> } />
        </Routes>
    }
}

#[component]
fn FilterView(
    filter: TodoFilter,
    set_filter: WriteSignal<TodoFilter>,
) -> impl IntoView {
    Effect::new(move |_| {
        set_filter.set(filter);
    });

    view! { <TodoListPlaceholder /> }
}

#[component]
fn TodoListPlaceholder() -> impl IntoView {
    let filter = use_todo_filter_signal();

    view! {
        <section class="main">
            <ul class="todo-list">
                <li class="todo">
                    <div class="view">
                        <label>{move || format!("Current filter: {}", filter.get().label())}</label>
                    </div>
                </li>
            </ul>
        </section>
    }
}
