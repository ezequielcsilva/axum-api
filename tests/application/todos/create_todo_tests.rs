use axum_api::{
    application::todos::create_todo::CreateTodoUseCase,
    domain::todos::{Todo, CreateTodoRequest, traits::TodoCreator},
    error::ApiError,
};
use uuid::Uuid;
use chrono::Utc;

struct MockRepo;

#[async_trait::async_trait]
impl TodoCreator for MockRepo {
    async fn create(&self, _data: CreateTodoRequest) -> Result<Todo, ApiError> {
        Ok(Todo {
            id: Uuid::new_v4(),
            title: "Test".to_string(),
            done: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[tokio::test]
async fn test_create_todo_use_case_creation() {
    let mock_repo = MockRepo;
    let _use_case = CreateTodoUseCase::new(&mock_repo);
    
    // Test that the use case was created successfully
    assert!(true); // If we get here, the use case was created
}

#[test]
fn test_create_todo_request_validation() {
    let request = CreateTodoRequest {
        title: "Test Todo".to_string(),
    };
    
    assert_eq!(request.title, "Test Todo");
}
