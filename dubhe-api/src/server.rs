use axum::{
    self,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tracing::info;

use crate::config::ApiConfig;
use crate::router;

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
        let addr: SocketAddr = self.cfg.listener_addr.parse().expect("msg");

        let api_router = router::api_router();
        let mut app = Router::new().nest("/api/v1/hello", api_router);
        if self.cfg.web.enable {
            app = app.fallback(static_handler);
            info!("api service web enable, listened in: http://{}", addr);
        }

        info!("api service enable, listened in: http://{}/api/v1", addr);
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
