use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    api::{dto::{CreateTodoRequest, TodoDto}, error::ApiError},
    todos::repo,
};

use super::dto::UpdateTodoRequest;

pub fn router() -> Router<crate::state::AppState> {
    Router::new().route("/todos", get(list_todos).post(create_todo))
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

async fn list_todos(
    State(pool): State<PgPool>,
    Query(query): Query<ListTodosQuery>,
) -> Result<Json<Vec<TodoDto>>, ApiError> {
    let todos = repo::list_all(&pool).await?;
    let todos = todos
        .into_iter()
        .filter(|todo| match query.filter {
            Some(TodoFilter::Active) => !todo.completed,
            Some(TodoFilter::Completed) => todo.completed,
            None => true,
        })
        .map(TodoDto::from)
        .collect();

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoDto>), ApiError> {
    let todo = repo::create(&pool, &payload.title, payload.position).await?;

    Ok((StatusCode::CREATED, Json(TodoDto::from(todo))))
}

#[allow(dead_code)]
fn _keep_update_request_compiled(_: UpdateTodoRequest) {}
