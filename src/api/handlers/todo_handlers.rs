use axum::{extract::{Path, State, Query}, Json};
use uuid::Uuid;

use crate::{
    state::AppState, 
    domain::todos::{
        Todo, CreateTodoRequest, UpdateTodoRequest, PaginationQuery, PaginatedResponse
    }, 
    domain::todos::traits::TodoFinder,
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
