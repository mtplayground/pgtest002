use serde::{Deserialize, Serialize};

use crate::todos::model::Todo;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: Option<i64>,
}

impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            position: todo.position,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub position: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub position: Option<i64>,
}
