use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../dubhe-web/dist/"]
struct Assets;

pub fn api_router(web_enable: bool) -> Router {
    let mut r = Router::new().route("/api/v1/ping", get(|| async { "pong" }));

    if web_enable {
        r = r.fallback(static_handler);
    }
    return r;
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
            // if path.contains('.') {
            //     return not_found().await;
            // }
            // index_html().await
            not_found().await
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
