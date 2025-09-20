use axum_api::domain::todos::{
    Todo, CreateTodoRequest, UpdateTodoRequest, PaginationQuery
};
use uuid::Uuid;
use chrono::Utc;

fn create_test_todo() -> Todo {
    Todo {
        id: Uuid::new_v4(),
        title: "Test Todo".to_string(),
        done: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn test_create_todo_request() {
    let request = CreateTodoRequest {
        title: "Test Todo".to_string(),
    };
    assert_eq!(request.title, "Test Todo");
}

#[test]
fn test_update_todo_request() {
    let request = UpdateTodoRequest {
        title: Some("Updated Todo".to_string()),
        done: Some(true),
    };
    assert_eq!(request.title, Some("Updated Todo".to_string()));
    assert_eq!(request.done, Some(true));
}

#[test]
fn test_pagination_query() {
    let query = PaginationQuery {
        page: 1,
        limit: 10,
    };
    assert_eq!(query.page, 1);
    assert_eq!(query.limit, 10);
}

#[test]
fn test_todo_entity() {
    let todo = create_test_todo();
    assert_eq!(todo.title, "Test Todo");
    assert_eq!(todo.done, false);
}
