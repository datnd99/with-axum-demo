
use crate::constant::{DB_DATABASE_NAME, DB_HOST, DB_PASSWORD, DB_PORT, DB_USERNAME, SYS_HOST, SYS_PORT};

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
    pub db: DatabaseConfig
}

impl AppConfig {
    pub fn read() -> Self {
        let server: ServerConfig = ServerConfig {
                addr: SYS_HOST.to_string(),
                port: *SYS_PORT,
            };

        let db: DatabaseConfig = DatabaseConfig {
            database_name: DB_DATABASE_NAME.to_string(),
            host: DB_HOST.to_string(),
            max_connections: 100,
            password: DB_PASSWORD.to_string(),
            port: *DB_PORT,
            username: DB_USERNAME.to_string()
        };

        Self {
            server,
            db
        }
    }
}
