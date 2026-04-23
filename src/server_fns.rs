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

#[server(UpdateTitle, "/api")]
pub async fn update_title(
    id: i64,
    title: String,
) -> Result<Option<TodoDto>, ServerFnError<String>> {
    let title = title.trim().to_string();

    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        if title.is_empty() {
            let deleted = repo::delete(&state.pool, id)
                .await
                .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

            if !deleted {
                return Err(ServerFnError::ServerError(format!(
                    "todo with id {id} was not found"
                )));
            }

            return Ok(None);
        }

        let todo = repo::update_title(&state.pool, id, &title)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        let todo = todo.ok_or_else(|| {
            ServerFnError::ServerError(format!("todo with id {id} was not found"))
        })?;

        Ok(Some(TodoDto::from(todo)))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, title);
        Err(ServerFnError::ServerError(
            "update_title server function can only run on the server".to_string(),
        ))
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        let deleted = repo::delete(&state.pool, id)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        if !deleted {
            return Err(ServerFnError::ServerError(format!(
                "todo with id {id} was not found"
            )));
        }

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        Err(ServerFnError::ServerError(
            "delete_todo server function can only run on the server".to_string(),
        ))
    }
}

#[server(ToggleAll, "/api")]
pub async fn toggle_all(completed: bool) -> Result<(), ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        repo::toggle_all(&state.pool, completed)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = completed;
        Err(ServerFnError::ServerError(
            "toggle_all server function can only run on the server".to_string(),
        ))
    }
}

#[server(ClearCompleted, "/api")]
pub async fn clear_completed() -> Result<(), ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        use crate::{state::AppState, todos::repo};

        let state = use_context::<AppState>()
            .ok_or_else(|| ServerFnError::ServerError("missing app state".to_string()))?;

        repo::clear_completed(&state.pool)
            .await
            .map_err(|error| ServerFnError::ServerError(error.to_string()))?;

        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "clear_completed server function can only run on the server".to_string(),
        ))
    }
}
