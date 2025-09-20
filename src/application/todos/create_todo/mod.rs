
use crate::domain::todos::{Todo, CreateTodoRequest};
use crate::domain::todos::traits::TodoCreator;
use crate::error::ApiError;

pub struct CreateTodoUseCase<'a, T: TodoCreator> {
    todo_repository: &'a T,
}

impl<'a, T: TodoCreator> CreateTodoUseCase<'a, T> {
    pub fn new(todo_repository: &'a T) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, request: CreateTodoRequest) -> Result<Todo, ApiError> {
        self.todo_repository.create(request).await
    }
}
