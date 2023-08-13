use axum::{
    self,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tracing::info;

use super::ApiConfig;
pub struct ApiServer {
    cfg: ApiConfig,
}

static INDEX_HTML: &str = "index.html";
#[derive(RustEmbed)]
#[folder = "../dubhe-web/dist/"]
struct Assets;

impl ApiServer {
    pub fn new(cfg: ApiConfig) -> ApiServer {
        ApiServer { cfg }
    }
    pub async fn run(&self) {
        let app = Router::new()
            .fallback(static_handler)
            .route("/api/v1/hello", get(|| async { "Hello, World!" }));
        let addr: SocketAddr = self.cfg.listener_addr.parse().expect("msg");
        info!(
            "api service enable, listened in: {}",
            self.cfg.listener_addr
        );
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }
            index_html().await
        }
    }
}
async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
