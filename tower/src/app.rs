use tokio::runtime::Runtime;

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
    pub fn run() {}
}
