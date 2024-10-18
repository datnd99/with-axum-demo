use std::sync::Arc;

use crate::{configure::AppConfig, error::AppResult, init::database::{DatabaseClient, DatabaseClientExt}};

// Tạo trạng thái để chia sẻ giữa các thread của hệ thống
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self>{
        let db: Arc<sea_orm::DatabaseConnection> =  Arc::new(DatabaseClient::build_from_config(&config).await?);

        Ok(Self {
            config: Arc::new(config),
            db
        })
    }
}