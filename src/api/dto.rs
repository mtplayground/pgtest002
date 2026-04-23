use serde::{Deserialize, Serialize};

use super::error::ApiError;
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

impl CreateTodoRequest {
    pub fn validated(self) -> Result<Self, ApiError> {
        let title = self.title.trim().to_string();

        if title.is_empty() {
            return Err(ApiError::UnprocessableEntity(
                "title must not be empty".to_string(),
            ));
        }

        Ok(Self {
            title,
            position: self.position,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub position: Option<i64>,
}

impl UpdateTodoRequest {
    pub fn validated_title(&self) -> Result<Option<String>, ApiError> {
        match self.title.as_deref() {
            Some(title) => {
                let title = title.trim().to_string();

                if title.is_empty() {
                    Err(ApiError::UnprocessableEntity(
                        "title must not be empty".to_string(),
                    ))
                } else {
                    Ok(Some(title))
                }
            }
            None => Ok(None),
        }
    }
}
