use sqlx::types::time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
