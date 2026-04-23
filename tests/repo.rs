use sqlx::PgPool;

use pgtest002::todos::repo;

#[sqlx::test]
async fn todo_repository_crud_and_bulk_operations(pool: PgPool) -> sqlx::Result<()> {
    let first = repo::create(&pool, "first todo", Some(1)).await?;
    let second = repo::create(&pool, "second todo", Some(2)).await?;

    let listed = repo::list_all(&pool).await?;
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, first.id);
    assert_eq!(listed[1].id, second.id);

    let updated = repo::update_title(&pool, first.id, "renamed todo")
        .await?
        .expect("created todo should be updateable");
    assert_eq!(updated.title, "renamed todo");

    let completed = repo::set_completed(&pool, first.id, true)
        .await?
        .expect("created todo should be completable");
    assert!(completed.completed);

    let toggled_count = repo::toggle_all(&pool, true).await?;
    assert_eq!(toggled_count, 1);

    let completed_todos = repo::list_all(&pool).await?;
    assert!(completed_todos.iter().all(|todo| todo.completed));

    let cleared_count = repo::clear_completed(&pool).await?;
    assert_eq!(cleared_count, 2);
    assert!(repo::list_all(&pool).await?.is_empty());

    let remaining = repo::create(&pool, "delete me", None).await?;
    assert!(repo::delete(&pool, remaining.id).await?);
    assert!(!repo::delete(&pool, remaining.id).await?);

    Ok(())
}
