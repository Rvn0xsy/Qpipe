
use clap::Parser;
use log::{info};
use crate::command::run_process_group;
use cli::Cli;

mod config;
mod models;
mod cli;
mod command;
mod run;
mod server;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // env_logger::init();
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let global = config::Config::new(&cli);
    let config = global.unwrap();
    run_process_group(config.process_group.clone());
    let mut workflow_server = server::WorkflowServer::new(config.clone());
    workflow_server.start_server().unwrap();
    info!("[+]Starting server on {}", config.server);
    workflow_server.handle_request().await;
}
