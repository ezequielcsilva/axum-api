use std::sync::Arc;
use sqlx::PgPool;

use crate::infrastructure::database::repositories::PostgresTodoRepository;

#[derive(Clone)]
pub struct AppState {
    pub todo_repository: Arc<PostgresTodoRepository>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            todo_repository: Arc::new(PostgresTodoRepository::new(pool)),
        }
    }
}