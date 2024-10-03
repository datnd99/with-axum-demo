use self::server::ServerConfig;
use serde::Deserialize;

pub mod server;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn read() -> Self {
        Self {
            server: ServerConfig {
                addr: "127.0.0.1".to_string(),
                port: 8080,
            },
        }
    }
}
