use uuid::Uuid;

use crate::domain::todos::Todo;
use crate::domain::todos::traits::TodoFinder;
use crate::error::ApiError;

pub struct GetTodoUseCase<'a, T: TodoFinder> {
    todo_repository: &'a T,
}

impl<'a, T: TodoFinder> GetTodoUseCase<'a, T> {
    pub fn new(todo_repository: &'a T) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<Todo>, ApiError> {
        self.todo_repository.find_by_id(id).await
    }
}
