use clap::Parser;
use std::fs;
use tracing::info;
use tracing_subscriber;

mod api;
mod app;
mod config;
mod flag;
use crate::flag::CliFlag;

fn main() {
    let cli = CliFlag::parse();

    // let t_builder = tracing_subscriber::fmt()
    //     .pretty()
    //     .with_line_number(false)
    //     .with_file(false)
    //     .with_thread_ids(true)
    //     .with_thread_names(true);
    // t_builder.try_init().expect("msg");
    tracing_subscriber::fmt::init();

    let path = cli.config.unwrap();

    info!("config path: {:?}", path);

    let content = fs::read_to_string(path).expect("read config path");
    let config_data: config::Config =
        serde_yaml::from_str(content.as_str()).expect("serialize config failed");

    app::AppBuilder::new(config_data).build().run();
}
