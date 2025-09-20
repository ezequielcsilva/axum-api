use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::todos::{Todo, CreateTodoRequest, UpdateTodoRequest, PaginationQuery, PaginatedResponse};
use crate::error::ApiError;

#[async_trait]
pub trait TodoCreator {
    async fn create(&self, data: CreateTodoRequest) -> Result<Todo, ApiError>;
}

#[async_trait]
pub trait TodoFinder {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, ApiError>;
    async fn find_by_done(&self, done: bool) -> Result<Vec<Todo>, ApiError>;
}

#[async_trait]
pub trait TodoPaginator {
    async fn find_all_paginated(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Todo>, ApiError>;
}

#[async_trait]
pub trait TodoUpdater {
    async fn update(&self, id: Uuid, data: UpdateTodoRequest) -> Result<Todo, ApiError>;
}

#[async_trait]
pub trait TodoDeleter {
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

