use crate::constant::ENV_VARS;

use self::server::ServerConfig;
use db::DatabaseConfig;
use serde::Deserialize;

pub mod server;
pub mod tracing;
pub mod db;

// định nghĩa config cho hệ thống
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: String
}

impl AppConfig {
    pub fn read() -> Self {
        Self {
            server: ServerConfig {
                addr: ENV_VARS.host.clone(),
                port: ENV_VARS.port,
            },
            db: DatabaseConfig::create_url()
        }
    }
}
