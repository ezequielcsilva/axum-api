use uuid::Uuid;

use crate::domain::todos::traits::TodoDeleter;
use crate::error::ApiError;

pub struct DeleteTodoUseCase<'a, T: TodoDeleter> {
    todo_repository: &'a T,
}

impl<'a, T: TodoDeleter> DeleteTodoUseCase<'a, T> {
    pub fn new(todo_repository: &'a T) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ApiError> {
        self.todo_repository.delete(id).await
    }
}
