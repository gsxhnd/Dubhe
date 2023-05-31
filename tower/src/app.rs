use axum::{routing::get, Router};
use futures::future;
use tokio::runtime::Runtime;
use tracing::info;

use tower_mqtt::MqttServer;
use tower_raft::raft_server;

use crate::config::Config;

pub struct App {
    runtime: Runtime,
    config: Config,
}

impl App {
    pub fn build(cfg: Config) -> Self {
        App {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                // .worker_threads(2)
                .build()
                .expect(""),
            config: cfg,
        }
    }
    pub fn run(&self) {
        let peer_config = self.config.peer.clone();
        let mqtt_config = self.config.mqtt.clone();
        let raft_handle = self.runtime.spawn(async {
            let raft = raft_server::RaftServer::new(peer_config);
            raft.run().await;
        });

        let mq_handle = self.runtime.spawn(async {
            let s = MqttServer::new(mqtt_config);
            s.run().await;
        });

        let api_handle = self.runtime.spawn(async {
            let app = Router::new().route("/", get(|| async { "Hello, World!" }));
            info!("api service started in: {}", "0.0.0.0:3000");
            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        let servers = vec![raft_handle, mq_handle, api_handle];
        self.runtime.block_on(async move {
            future::join_all(servers).await;
        });
    }
}
