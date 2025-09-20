use axum::{
    routing::{get, post},
    Router,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use crate::{api::handlers, doc::ApiDoc, state::AppState};
use crate::api::handlers::todo_handlers;

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/todos", post(handlers::create_todo).get(handlers::list_todos))
        .route("/todos/:id", get(handlers::get_todo).put(handlers::update_todo).delete(handlers::delete_todo))
        .route("/todos/done/:done", get(handlers::get_todos_by_done))
        .route("/todos/performance-test", post(todo_handlers::performance_test))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .with_state(state)
}