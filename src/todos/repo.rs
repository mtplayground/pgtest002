use sqlx::{PgPool, query, query_as};

use super::model::Todo;

pub async fn list_all(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    query_as::<_, Todo>(
        r#"
        SELECT
            id,
            title,
            completed,
            position,
            created_at,
            updated_at
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
    query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, position)
        VALUES ($1, $2)
        RETURNING
            id,
            title,
            completed,
            position,
            created_at,
            updated_at
        "#
    )
    .bind(title)
    .bind(position)
    .fetch_one(pool)
    .await
}

pub async fn update_title(
    pool: &PgPool,
    id: i64,
    title: &str,
) -> Result<Option<Todo>, sqlx::Error> {
    query_as::<_, Todo>(
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
            created_at,
            updated_at
        "#
    )
    .bind(id)
    .bind(title)
    .fetch_optional(pool)
    .await
}

pub async fn set_completed(
    pool: &PgPool,
    id: i64,
    completed: bool,
) -> Result<Option<Todo>, sqlx::Error> {
    query_as::<_, Todo>(
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
            created_at,
            updated_at
        "#
    )
    .bind(id)
    .bind(completed)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = query(
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn toggle_all(pool: &PgPool, completed: bool) -> Result<u64, sqlx::Error> {
    let result = query(
        r#"
        UPDATE todos
        SET completed = $1,
            updated_at = NOW()
        WHERE completed IS DISTINCT FROM $1
        "#
    )
    .bind(completed)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn clear_completed(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = query(
        r#"
        DELETE FROM todos
        WHERE completed = TRUE
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
