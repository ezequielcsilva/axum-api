use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    pub title: String
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, ToSchema)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 10 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_query_defaults() {
        let query = PaginationQuery {
            page: 0,
            limit: 0,
        };
        
        assert_eq!(query.page, 0);
        assert_eq!(query.limit, 0);
    }

    #[test]
    fn test_pagination_meta_calculation() {
        let meta = PaginationMeta {
            page: 2,
            limit: 10,
            total: 25,
            total_pages: 3,
            has_next: true,
            has_prev: true,
        };
        
        assert_eq!(meta.page, 2);
        assert_eq!(meta.limit, 10);
        assert_eq!(meta.total, 25);
        assert_eq!(meta.total_pages, 3);
        assert!(meta.has_next);
        assert!(meta.has_prev);
    }

    #[test]
    fn test_pagination_meta_first_page() {
        let meta = PaginationMeta {
            page: 1,
            limit: 10,
            total: 25,
            total_pages: 3,
            has_next: true,
            has_prev: false,
        };
        
        assert!(!meta.has_prev);
        assert!(meta.has_next);
    }

    #[test]
    fn test_pagination_meta_last_page() {
        let meta = PaginationMeta {
            page: 3,
            limit: 10,
            total: 25,
            total_pages: 3,
            has_next: false,
            has_prev: true,
        };
        
        assert!(meta.has_prev);
        assert!(!meta.has_next);
    }
}