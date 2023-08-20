use axum::routing::get;
use axum::Router;

pub fn api_router() -> Router {
    Router::new().route("/", get(root))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
