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
        let api_router = router::api_router(self.cfg.web.enable);
        self.output().await;

        axum::Server::bind(&self.addr)
            .serve(api_router.into_make_service())
            .await
            .unwrap();
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
