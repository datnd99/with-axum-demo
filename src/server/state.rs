use std::sync::Arc;

use crate::configure::AppConfig;

// Tạo trạng thái để chia sẻ giữa các thread của hệ thống
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