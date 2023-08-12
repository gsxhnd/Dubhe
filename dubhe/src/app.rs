use futures::future;
use tokio::runtime::Runtime;

use dubhe_mqtt::MqttServer;
use dubhe_raft::RaftServer;

use crate::api::{ApiConfig, ApiServer};
use crate::config::Config;

#[derive(Debug)]
pub struct App {
    runtime: Runtime,
    raft_config: dubhe_raft::RaftConfig,
    mqtt_config: dubhe_mqtt::MqttConfig,
    api_config: ApiConfig,
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
            api_config: self.config.api.clone(),
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
            ApiServer::new(self.api_config).run().await;
        });
        let services = vec![raft_handle, mq_handle, api_handle];
        self.runtime.block_on(async {
            future::join_all(services).await;
        });
    }
}
