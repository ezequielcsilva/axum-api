use axum::{
    routing::{get, post, put, delete},
    Router,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use crate::{api::handlers, doc::ApiDoc, state::AppState};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/todos", post(handlers::create_todo).get(handlers::list_todos))
        .route("/todos/:id", get(handlers::get_todo).put(handlers::update_todo).delete(handlers::delete_todo))
        .route("/todos/done/:done", get(handlers::get_todos_by_done))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .with_state(state)
}