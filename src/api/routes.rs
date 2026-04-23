use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    api::{dto::{CreateTodoRequest, TodoDto}, error::ApiError},
    todos::repo,
};

use super::dto::UpdateTodoRequest;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", patch(update_todo).delete(delete_todo))
        .route("/todos/toggle-all", post(toggle_all_todos))
        .route("/todos/completed", delete(clear_completed_todos))
}

#[derive(Debug, Deserialize)]
struct ListTodosQuery {
    filter: Option<TodoFilter>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum TodoFilter {
    Active,
    Completed,
}

#[derive(Debug, Deserialize)]
struct ToggleAllRequest {
    completed: bool,
}

async fn list_todos(
    State(pool): State<PgPool>,
    Query(query): Query<ListTodosQuery>,
) -> Result<Json<Vec<TodoDto>>, ApiError> {
    Ok(Json(filtered_todos(&pool, query.filter).await?))
}

async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoDto>), ApiError> {
    let payload = payload.validated()?;
    let todo = repo::create(&pool, &payload.title, payload.position).await?;

    Ok((StatusCode::CREATED, Json(TodoDto::from(todo))))
}

async fn update_todo(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoDto>, ApiError> {
    let id = parse_todo_id(&id)?;

    if payload.position.is_some() {
        return Err(ApiError::BadRequest(
            "position updates are not supported by this endpoint".to_string(),
        ));
    }

    let title = payload.validated_title()?;
    let mut updated_todo = None;

    if let Some(title) = title.as_deref() {
        updated_todo = Some(
            repo::update_title(&pool, id, title)
                .await?
                .ok_or_else(|| ApiError::NotFound(format!("todo {id} not found")))?,
        );
    }

    if let Some(completed) = payload.completed {
        updated_todo = Some(
            repo::set_completed(&pool, id, completed)
                .await?
                .ok_or_else(|| ApiError::NotFound(format!("todo {id} not found")))?,
        );
    }

    let todo = updated_todo.ok_or_else(|| {
        ApiError::BadRequest("update requires title and/or completed".to_string())
    })?;

    Ok(Json(TodoDto::from(todo)))
}

async fn delete_todo(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, ApiError> {
    let id = parse_todo_id(&id)?;

    if repo::delete(&pool, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound(format!("todo {id} not found")))
    }
}

async fn toggle_all_todos(
    State(pool): State<PgPool>,
    Json(payload): Json<ToggleAllRequest>,
) -> Result<Json<Vec<TodoDto>>, ApiError> {
    repo::toggle_all(&pool, payload.completed).await?;

    Ok(Json(filtered_todos(&pool, None).await?))
}

async fn clear_completed_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<TodoDto>>, ApiError> {
    repo::clear_completed(&pool).await?;

    Ok(Json(filtered_todos(&pool, None).await?))
}

async fn filtered_todos(
    pool: &PgPool,
    filter: Option<TodoFilter>,
) -> Result<Vec<TodoDto>, ApiError> {
    let todos = repo::list_all(pool).await?;

    Ok(todos
        .into_iter()
        .filter(|todo| match filter {
            Some(TodoFilter::Active) => !todo.completed,
            Some(TodoFilter::Completed) => todo.completed,
            None => true,
        })
        .map(TodoDto::from)
        .collect())
}

fn parse_todo_id(raw_id: &str) -> Result<i64, ApiError> {
    raw_id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("invalid todo id".to_string()))
}
