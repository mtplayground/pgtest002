use leptos::prelude::*;

use crate::api::dto::{CreateTodoRequest, TodoDto};

#[server(CreateTodo, "/api")]
pub async fn create_todo(title: String) -> Result<TodoDto, ServerFnError<String>> {
    let request = CreateTodoRequest {
        title,
        position: None,
    }
    .validated()
    .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        let todo = repo::create(&state.pool, &request.title, request.position)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        Ok(TodoDto::from(todo))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = request;
        Err(ServerFnError::ServerError(
            "create_todo server function can only run on the server".to_string(),
        ))
    }
}

#[server(SetCompleted, "/api")]
pub async fn set_completed(
    id: i64,
    completed: bool,
) -> Result<TodoDto, ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        let todo = repo::set_completed(&state.pool, id, completed)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        let todo = todo.ok_or_else(|| {
            ServerFnError::ServerError(format!("todo with id {id} was not found"))
        })?;

        Ok(TodoDto::from(todo))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, completed);
        Err(ServerFnError::ServerError(
            "set_completed server function can only run on the server".to_string(),
        ))
    }
}
