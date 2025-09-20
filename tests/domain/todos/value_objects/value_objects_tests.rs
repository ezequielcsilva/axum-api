use axum_api::domain::todos::value_objects::{
    CreateTodoRequest, UpdateTodoRequest, PaginationQuery, 
    PaginatedResponse, PaginationMeta
};

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
fn test_update_todo_request_partial() {
    let request = UpdateTodoRequest {
        title: Some("Updated Todo".to_string()),
        done: None,
    };

    assert_eq!(request.title, Some("Updated Todo".to_string()));
    assert_eq!(request.done, None);
}

#[test]
fn test_pagination_query_defaults() {
    let query = PaginationQuery {
        page: 0,
        limit: 0,
    };

    // Test default values
    assert_eq!(axum_api::domain::todos::value_objects::default_page(), 1);
    assert_eq!(axum_api::domain::todos::value_objects::default_limit(), 10);
}

#[test]
fn test_pagination_meta() {
    let meta = PaginationMeta {
        page: 1,
        limit: 10,
        total: 25,
        total_pages: 3,
        has_next: true,
        has_prev: false,
    };

    assert_eq!(meta.page, 1);
    assert_eq!(meta.limit, 10);
    assert_eq!(meta.total, 25);
    assert_eq!(meta.total_pages, 3);
    assert!(meta.has_next);
    assert!(!meta.has_prev);
}

#[test]
fn test_paginated_response() {
    let todos = vec![
        "Todo 1".to_string(),
        "Todo 2".to_string(),
    ];
    
    let meta = PaginationMeta {
        page: 1,
        limit: 10,
        total: 2,
        total_pages: 1,
        has_next: false,
        has_prev: false,
    };

    let response = PaginatedResponse {
        data: todos,
        pagination: meta,
    };

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.pagination.total, 2);
}
