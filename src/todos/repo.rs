use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;

use super::model::Todo;

pub async fn list_all(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        SELECT
            id,
            title,
            completed,
            position,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        FROM todos
        ORDER BY position NULLS LAST, id ASC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    title: &str,
    position: Option<i64>,
) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, position)
        VALUES ($1, $2)
        RETURNING
            id,
            title,
            completed,
            position,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        "#,
        title,
        position
    )
    .fetch_one(pool)
    .await
}

pub async fn update_title(
    pool: &PgPool,
    id: i64,
    title: &str,
) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            title,
            completed,
            position,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        "#,
        id,
        title
    )
    .fetch_optional(pool)
    .await
}

pub async fn set_completed(
    pool: &PgPool,
    id: i64,
    completed: bool,
) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET completed = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            title,
            completed,
            position,
            created_at as "created_at!: OffsetDateTime",
            updated_at as "updated_at!: OffsetDateTime"
        "#,
        id,
        completed
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn toggle_all(pool: &PgPool, completed: bool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE todos
        SET completed = $1,
            updated_at = NOW()
        WHERE completed IS DISTINCT FROM $1
        "#,
        completed
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn clear_completed(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE completed = TRUE
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
