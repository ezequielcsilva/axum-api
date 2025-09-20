#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "OK"))
)]
pub async fn health() -> &'static str { 
    "ok" 
}
