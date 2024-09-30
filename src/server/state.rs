use std::sync::Arc;

use crate::configure::AppConfig;


#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self{
        Self {
            config: Arc::new(config)
        }
    }
}