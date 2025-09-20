use axum::{extract::{Path, State, Query}, Json};
use uuid::Uuid;
use tokio::time::Instant;
use serde::{Deserialize, Serialize};

use crate::{
    state::AppState, 
    domain::todos::{
        Todo, CreateTodoRequest, UpdateTodoRequest, PaginationQuery, PaginatedResponse
    }, 
    domain::todos::traits::{TodoFinder, TodoCreator},
    application::todos::{
        CreateTodoUseCase, GetTodoUseCase, ListTodosUseCase, UpdateTodoUseCase, DeleteTodoUseCase
    },
    error::ApiError
};

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodoRequest,
    responses((status = 201, body = Todo)),
    tag = "todos"
)]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, ApiError> {
    let use_case = CreateTodoUseCase::new(&*state.todo_repository);
    let todo = use_case.execute(payload).await?;
    Ok(Json(todo))
}

#[utoipa::path(
    get,
    path = "/todos",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10, max: 100)")
    ),
    responses((status = 200, body = PaginatedResponse<Todo>)),
    tag = "todos"
)]
pub async fn list_todos(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Todo>>, ApiError> {
    let use_case = ListTodosUseCase::new(&*state.todo_repository);
    let result = use_case.execute(pagination).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    responses((status = 200, body = Todo), (status = 404, description = "not found")),
    tag = "todos"
)]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, ApiError> {
    let use_case = GetTodoUseCase::new(&*state.todo_repository);
    let todo = use_case.execute(id).await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(todo))
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    request_body = UpdateTodoRequest,
    responses((status = 200, body = Todo), (status = 404, description = "not found")),
    tag = "todos"
)]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, ApiError> {
    let use_case = UpdateTodoUseCase::new(&*state.todo_repository);
    let todo = use_case.execute(id, payload).await?;
    Ok(Json(todo))
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo ID")),
    responses((status = 204, description = "deleted"), (status = 404, description = "not found")),
    tag = "todos"
)]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let use_case = DeleteTodoUseCase::new(&*state.todo_repository);
    use_case.execute(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/todos/done/{done}",
    params(("done" = bool, Path, description = "Filter by done status")),
    responses((status = 200, body = [Todo])),
    tag = "todos"
)]
pub async fn get_todos_by_done(
    State(state): State<AppState>,
    Path(done): Path<bool>,
) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = state.todo_repository.find_by_done(done).await?;
    Ok(Json(todos))
}

#[derive(Deserialize)]
pub struct PerformanceTestRequest {
    pub message_count: u32,
    pub batch_size: Option<u32>,
}

#[derive(Serialize)]
pub struct PerformanceTestResponse {
    pub total_messages: u32,
    pub duration_ms: u64,
    pub messages_per_second: f64,
    pub success_count: u32,
    pub error_count: u32,
    pub success_rate: f64,
}

#[utoipa::path(
    post,
    path = "/todos/performance-test",
    request_body = PerformanceTestRequest,
    responses((status = 200, body = PerformanceTestResponse)),
    tag = "todos"
)]
pub async fn performance_test(
    State(state): State<AppState>,
    Json(payload): Json<PerformanceTestRequest>,
) -> Result<Json<PerformanceTestResponse>, ApiError> {
    let start_time = Instant::now();
    let message_count = payload.message_count;
    let batch_size = payload.batch_size.unwrap_or(10).min(100);
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    // Create todos directly in batches
    for batch_start in (0..message_count).step_by(batch_size as usize) {
        let batch_end = (batch_start + batch_size).min(message_count);
        
        for i in batch_start..batch_end {
            let create_request = CreateTodoRequest {
                title: format!("Performance Test Todo #{}", i),
                done: Some(i % 2 == 0),
            };
            
            // Create todo directly using the repository
            match state.todo_repository.create(create_request).await {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }
        
        // Small delay between batches to avoid overwhelming the database
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
    
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis() as u64;
    let messages_per_second = (message_count as f64) / (duration.as_secs_f64());
    let success_rate = (success_count as f64 / message_count as f64) * 100.0;
    
    let response = PerformanceTestResponse {
        total_messages: message_count,
        duration_ms,
        messages_per_second,
        success_count,
        error_count,
        success_rate,
    };
    
    Ok(Json(response))
}
