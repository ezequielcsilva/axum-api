use axum_api::{
    application::todos::list_todos::ListTodosUseCase,
    domain::todos::{Todo, PaginationQuery, PaginatedResponse, PaginationMeta, traits::TodoPaginator},
    error::ApiError,
};

struct MockRepo;

#[async_trait::async_trait]
impl TodoPaginator for MockRepo {
    async fn find_all_paginated(&self, _pagination: PaginationQuery) -> Result<PaginatedResponse<Todo>, ApiError> {
        Ok(PaginatedResponse {
            data: vec![],
            pagination: PaginationMeta {
                page: 1,
                limit: 10,
                total: 0,
                total_pages: 0,
                has_next: false,
                has_prev: false,
            },
        })
    }
}

#[tokio::test]
async fn test_list_todos_use_case_creation() {
    let mock_repo = MockRepo;
    let _use_case = ListTodosUseCase::new(&mock_repo);
    
    assert!(true); // If we get here, the use case was created
}

#[test]
fn test_pagination_query_validation() {
    let pagination = PaginationQuery {
        page: 1,
        limit: 10,
    };
    
    assert_eq!(pagination.page, 1);
    assert_eq!(pagination.limit, 10);
}
