mod app;
mod state;
mod error;
mod doc;
mod domain;
mod application;
mod infrastructure;
mod api;

use std::net::SocketAddr;
use app::build_app;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/axum_api".to_string());
    
    println!("Connecting to database: {}", database_url);
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    // Run migrations
    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed successfully!");

    // Create application state
    let state = AppState::new(pool);
    let app = build_app(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("→ Server running at http://{addr}");
    println!("→ Documentation available at http://{addr}/docs");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server started at http://{}", addr);
    axum::serve(listener, app).await?;
    
    Ok(())
}
