use serde::{Deserialize, Serialize};
use tokio::fs;
use anyhow::Result;
use reqwest as request;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_config: ServerConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: i64
}

async fn fetch_default_configuration(url: &str) -> Result<Config> {
    let response = request::get(url).await?;
    let content = response.text().await?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}


pub async fn init_config() -> Result<Config> {
    let config_path = "config.toml";
    let default_config_url = "https://raw.githubusercontent.com/your_username/your_repo/main/default_config.toml";

    
    if !fs::metadata(config_path).await.is_ok() {
        
        let default_config = fetch_default_configuration(default_config_url).await?;
        let toml_string = toml::to_string(&default_config)?;
        
        
        fs::write(config_path, toml_string).await?;
    }

    
    let config_str = fs::read_to_string(config_path).await?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}

