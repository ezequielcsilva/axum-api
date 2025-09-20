use axum_api::api::handlers::health::health;

#[tokio::test]
async fn test_health_endpoint() {
    let result = health().await;
    assert_eq!(result, "ok");
}
