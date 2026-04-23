use std::error::Error;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use leptos::config::get_configuration;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

use pgtest002::{
    api::{dto::TodoDto, routes},
    state::AppState,
};

#[sqlx::test]
async fn todo_api_crud_flow(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let app = test_app(pool)?;

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "write tests",
                        "position": 1
                    })
                    .to_string(),
                ))?,
        )
        .await?;
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created: TodoDto = response_json(create_response).await?;

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/todos?filter=active")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(list_response.status(), StatusCode::OK);
    let listed: Vec<TodoDto> = response_json(list_response).await?;
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);

    let update_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/todos/{}", created.id))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "write more tests",
                        "completed": true
                    })
                    .to_string(),
                ))?,
        )
        .await?;
    assert_eq!(update_response.status(), StatusCode::OK);
    let updated: TodoDto = response_json(update_response).await?;
    assert_eq!(updated.title, "write more tests");
    assert!(updated.completed);

    let completed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/todos?filter=completed")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(completed_response.status(), StatusCode::OK);
    let completed: Vec<TodoDto> = response_json(completed_response).await?;
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].id, created.id);

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/todos/{}", created.id))
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let empty_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/todos")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(empty_response.status(), StatusCode::OK);
    let remaining: Vec<TodoDto> = response_json(empty_response).await?;
    assert!(remaining.is_empty());

    Ok(())
}

#[sqlx::test]
async fn todo_api_bulk_endpoints_and_validation(pool: PgPool) -> Result<(), Box<dyn Error>> {
    let app = test_app(pool)?;

    create_todo(&app, "first").await?;
    create_todo(&app, "second").await?;

    let toggle_all_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos/toggle-all")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "completed": true }).to_string()))?,
        )
        .await?;
    assert_eq!(toggle_all_response.status(), StatusCode::OK);
    let toggled: Vec<TodoDto> = response_json(toggle_all_response).await?;
    assert_eq!(toggled.len(), 2);
    assert!(toggled.iter().all(|todo| todo.completed));

    let clear_completed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/todos/completed")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(clear_completed_response.status(), StatusCode::OK);
    let cleared: Vec<TodoDto> = response_json(clear_completed_response).await?;
    assert!(cleared.is_empty());

    let invalid_id_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/todos/not-a-number")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "completed": true }).to_string()))?,
        )
        .await?;
    assert_eq!(invalid_id_response.status(), StatusCode::BAD_REQUEST);
    let invalid_id_body: Value = response_json(invalid_id_response).await?;
    assert_eq!(invalid_id_body["error"], "invalid todo id");

    let invalid_title_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "   ",
                        "position": null
                    })
                    .to_string(),
                ))?,
        )
        .await?;
    assert_eq!(invalid_title_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let invalid_title_body: Value = response_json(invalid_title_response).await?;
    assert_eq!(invalid_title_body["error"], "title must not be empty");

    Ok(())
}

fn test_app(pool: PgPool) -> Result<Router, Box<dyn Error>> {
    let leptos_options = get_configuration(Some("Cargo.toml"))?.leptos_options;
    let state = AppState::new(pool, leptos_options);

    Ok(Router::new().nest("/api", routes::router()).with_state(state))
}

async fn create_todo(app: &Router, title: &str) -> Result<TodoDto, Box<dyn Error>> {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/todos")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "title": title,
                        "position": null
                    })
                    .to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    response_json(response).await
}

async fn response_json<T>(response: axum::response::Response) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok(serde_json::from_slice(&bytes)?)
}
