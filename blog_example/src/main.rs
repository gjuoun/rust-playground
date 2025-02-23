mod config;
mod health;
mod routes;

use config::EnvConfig;
use routes::user_routes;

#[tokio::main]
async fn main() {
    // Initialize environment configuration
    if let Err(e) = EnvConfig::init() {
        eprintln!("Failed to initialize config: {}", e);
        std::process::exit(1);
    }

    let config = EnvConfig::get_instance();

    println!("Config: {:#?}", config);

    // Build our application with routes
    let app = user_routes();

    // Run it
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        });

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    });
}
