use axum::{routing::get, Router};
use futures::future;
use tokio::runtime::Runtime;
use tracing::info;

use tower_mqtt::MqttServer;
use tower_raft::RaftServer;

use crate::config::Config;

#[derive(Debug)]
pub struct App {
    runtime: Runtime,
    raft_config: tower_raft::RaftConfig,
    mqtt_config: tower_mqtt::MqttConfig,
}

pub struct AppBuilder {
    config: Config,
}
impl AppBuilder {
    pub fn new(cfg: Config) -> Self {
        AppBuilder { config: cfg }
    }
    pub fn build(self) -> App {
        App {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                // .worker_threads(2)
                .build()
                .expect(""),
            raft_config: self.config.peer.clone(),
            mqtt_config: self.config.mqtt.clone(),
        }
    }
}

impl App {
    pub fn run(self) {
        let raft_handle = self.runtime.spawn(async {
            RaftServer::new(self.raft_config).run().await;
        });

        let mq_handle = self.runtime.spawn(async {
            MqttServer::new(self.mqtt_config).run().await;
        });

        let api_handle = self.runtime.spawn(async {
            let app = Router::new().route("/", get(|| async { "Hello, World!" }));
            info!("api service started in: {}", "0.0.0.0:3000");
            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        let services = vec![raft_handle, mq_handle, api_handle];
        self.runtime.block_on(async {
            future::join_all(services).await;
        });
    }
}
