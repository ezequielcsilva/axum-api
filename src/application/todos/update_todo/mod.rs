use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::todos::{Todo, UpdateTodoRequest};
use crate::domain::todos::traits::TodoUpdater;
use crate::error::ApiError;

pub struct UpdateTodoUseCase<'a, T: TodoUpdater> {
    todo_repository: &'a T,
}

impl<'a, T: TodoUpdater> UpdateTodoUseCase<'a, T> {
    pub fn new(todo_repository: &'a T) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, id: Uuid, request: UpdateTodoRequest) -> Result<Todo, ApiError> {
        self.todo_repository.update(id, request).await
    }
}
