use std::env;

use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub env: String,
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        if let Ok(env_file) = env::var("ENV_FILE") {
            dotenvy::from_filename(env_file).ok();
        } else {
            dotenv().ok();
        }
        envy::from_env::<Config>().expect("配置加载失败")
    }
}
