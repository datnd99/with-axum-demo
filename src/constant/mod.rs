use std::env;

use once_cell::sync::Lazy;

// khởi tạo cấu trúc cho server
pub struct EnvConfig {
    pub host: String,
    pub port: u16,
}

// đọc file .env và tải dữ liệu vào cấu trúc server
pub static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| {
    let port: u16 = env::var("PORT")
                    .ok()
                    .and_then(|port: String| port.parse::<u16>().ok())
                    .unwrap_or(8080);

    let host: String = env::var("HOST")
                        .ok()
                        .unwrap_or("127.0.0.1".to_string());

    EnvConfig {
        host,
        port,
    }
});



