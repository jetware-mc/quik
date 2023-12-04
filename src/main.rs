use crate::configuration::init_config;
use anyhow::Result;
use configuration::Config;
use logging::startup;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

pub mod logging;
pub mod configuration;

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}


#[tokio::main]
async fn main() -> Result<()> {
    let config_task = tokio::spawn(init_config());
    let config_result = config_task.await??;
    let config_option = Some(config_result);

    let mut config_guard = CONFIG.lock().await.unwrap();

    *config_guard = config_option;

    startup();

    Ok(())
}


