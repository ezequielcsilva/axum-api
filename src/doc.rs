use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
           paths(
               crate::api::handlers::health::health,
               crate::api::handlers::todo_handlers::create_todo,
               crate::api::handlers::todo_handlers::list_todos,
               crate::api::handlers::todo_handlers::get_todo,
               crate::api::handlers::todo_handlers::update_todo,
               crate::api::handlers::todo_handlers::delete_todo,
               crate::api::handlers::todo_handlers::get_todos_by_done
           ),
    components(
        schemas(
            crate::domain::todos::Todo,
            crate::domain::todos::CreateTodoRequest,
            crate::domain::todos::UpdateTodoRequest,
            crate::domain::todos::PaginationQuery,
            crate::domain::todos::PaginatedResponse<crate::domain::todos::Todo>,
            crate::domain::todos::PaginationMeta
        )
    ),
    tags((name = "todos", description = "Todo operations"))
)]
pub struct ApiDoc;