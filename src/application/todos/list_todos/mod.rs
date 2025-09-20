use async_trait::async_trait;

use crate::domain::todos::{Todo, PaginationQuery, PaginatedResponse};
use crate::domain::todos::traits::TodoPaginator;
use crate::error::ApiError;

pub struct ListTodosUseCase<'a, T: TodoPaginator> {
    todo_repository: &'a T,
}

impl<'a, T: TodoPaginator> ListTodosUseCase<'a, T> {
    pub fn new(todo_repository: &'a T) -> Self {
        Self { todo_repository }
    }

    pub async fn execute(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Todo>, ApiError> {
        self.todo_repository.find_all_paginated(pagination).await
    }
}
