use std::net::SocketAddr;
use tracing::info;

use crate::config::ApiConfig;
use crate::router;

pub struct ApiServer {
    cfg: ApiConfig,
    addr: SocketAddr,
}

impl ApiServer {
    pub fn new(cfg: ApiConfig) -> ApiServer {
        let addr: SocketAddr = cfg.listener_addr.parse().expect("msg");
        ApiServer { cfg, addr }
    }

    pub async fn run(&self) {
        let listen = tokio::net::TcpListener::bind(self.addr).await.unwrap();
        let api_router = router::api_router(self.cfg.web.enable);
        self.output().await;

        axum::serve(listen, api_router).await.unwrap();
    }

    async fn output(&self) {
        info!(
            "api service enable, listened in: http://{}/api/v1",
            self.addr
        );
        if self.cfg.web.enable {
            info!("api service web enable, listened in: http://{}", self.addr);
        }
    }
}
