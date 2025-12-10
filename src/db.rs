use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::config::Config;

pub async fn create_pool(config: &Config) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("数据库连接失败")
}
