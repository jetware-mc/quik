use std::time::Duration;

use crate::configuration::init_config;
use crate::shutdown::shutdown_task;
use anyhow::Result;
use configuration::Config;
use logging::startup;

use tokio_graceful_shutdown::SubsystemBuilder;
use tokio_graceful_shutdown::Toplevel;

pub mod logging;
pub mod configuration;
pub mod shutdown;

static mut CONFIG: Option<Config> = None;

pub fn get_config() -> &'static Config {
    unsafe {
        CONFIG.as_ref().unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_task = tokio::spawn(init_config());
    let config_result = config_task.await??;
    let config_option = Some(config_result);

    unsafe {
        CONFIG = config_option;
    }

    startup();
    point!("Running proxy on port {}...", get_config().server.port);

    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("shutdown_task", shutdown::shutdown_task(s)));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}


