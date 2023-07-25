use axum::{self, routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

use super::ApiConfig;
pub struct ApiServer {
    cfg: ApiConfig,
}

impl ApiServer {
    pub fn new(cfg: ApiConfig) -> ApiServer {
        ApiServer { cfg }
    }
    pub async fn run(&self) {
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));
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
