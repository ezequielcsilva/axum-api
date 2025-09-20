use axum_api::domain::todos::entities::todo::Todo;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_todo_creation() {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let todo = Todo {
        id,
        title: "Test Todo".to_string(),
        done: false,
        created_at: now,
        updated_at: now,
    };

    assert_eq!(todo.id, id);
    assert_eq!(todo.title, "Test Todo");
    assert_eq!(todo.done, false);
    assert_eq!(todo.created_at, now);
    assert_eq!(todo.updated_at, now);
}

#[test]
fn test_todo_serialization() {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let todo = Todo {
        id,
        title: "Test Todo".to_string(),
        done: false,
        created_at: now,
        updated_at: now,
    };

    let json = serde_json::to_string(&todo).unwrap();
    let deserialized: Todo = serde_json::from_str(&json).unwrap();

    assert_eq!(todo.id, deserialized.id);
    assert_eq!(todo.title, deserialized.title);
    assert_eq!(todo.done, deserialized.done);
}
