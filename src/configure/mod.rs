use crate::constant::ENV_VARS;

use self::server::ServerConfig;
use serde::Deserialize;

pub mod server;

// định nghĩa config cho hệ thống
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn read() -> Self {
        Self {
            server: ServerConfig {
                addr: ENV_VARS.host.clone(),
                port: ENV_VARS.port,
            },
        }
    }
}
