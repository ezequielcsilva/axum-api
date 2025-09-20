use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::todos::{Todo, CreateTodoRequest, UpdateTodoRequest, PaginationQuery, PaginatedResponse, PaginationMeta};
use crate::error::ApiError;
use crate::domain::todos::traits::{TodoCreator, TodoFinder, TodoPaginator, TodoUpdater, TodoDeleter};

pub struct PostgresTodoRepository {
    pool: PgPool,
}

impl PostgresTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TodoCreator for PostgresTodoRepository {
    async fn create(&self, data: CreateTodoRequest) -> Result<Todo, ApiError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (id, title, done, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, done, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&data.title)
        .bind(data.done.unwrap_or(false))
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(todo)
    }
}

#[async_trait::async_trait]
impl TodoFinder for PostgresTodoRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, ApiError> {
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, title, done, created_at, updated_at FROM todos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(todo)
    }

    async fn find_by_done(&self, done: bool) -> Result<Vec<Todo>, ApiError> {
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, done, created_at, updated_at FROM todos WHERE done = $1 ORDER BY created_at DESC"
        )
        .bind(done)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(todos)
    }
}

#[async_trait::async_trait]
impl TodoPaginator for PostgresTodoRepository {
    async fn find_all_paginated(&self, pagination: PaginationQuery) -> Result<PaginatedResponse<Todo>, ApiError> {
        // Validate pagination parameters
        let page = pagination.page.max(1);
        let limit = pagination.limit.min(100).max(1); // Max 100 items per page
        let offset = (page - 1) * limit;

        // Get total count
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM todos")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Get paginated results
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, done, created_at, updated_at FROM todos ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let total = total as u64;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        let has_next = page < total_pages;
        let has_prev = page > 1;

        let pagination_meta = PaginationMeta {
            page,
            limit,
            total,
            total_pages,
            has_next,
            has_prev,
        };

        Ok(PaginatedResponse {
            data: todos,
            pagination: pagination_meta,
        })
    }
}

#[async_trait::async_trait]
impl TodoUpdater for PostgresTodoRepository {
    async fn update(&self, id: Uuid, data: UpdateTodoRequest) -> Result<Todo, ApiError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos 
            SET title = COALESCE($1, title),
                done = COALESCE($2, done),
                updated_at = $3
            WHERE id = $4
            RETURNING id, title, done, created_at, updated_at
            "#
        )
        .bind(&data.title)
        .bind(data.done)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(todo)
    }
}

#[async_trait::async_trait]
impl TodoDeleter for PostgresTodoRepository {
    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
}
