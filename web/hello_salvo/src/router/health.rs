use salvo::prelude::*;

#[handler]
async fn health() -> &'static str {
    "health"
}

pub fn health_router() -> Router {
    Router::with_path("/health").get(health)
}
