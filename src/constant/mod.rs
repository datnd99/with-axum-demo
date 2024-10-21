use std::env;

use once_cell::sync::Lazy;

// Load biến môi trường cho hệ thống
pub static SYS_PORT: Lazy<u16> = Lazy::new(|| {
    env::var("PORT")
        .ok()
        .and_then(|port: String| port.parse::<u16>().ok())
        .unwrap_or(8080)
});

pub static SYS_HOST: Lazy<String> = Lazy::new(|| {
    env::var("HOST")
        .ok()
        .unwrap_or("127.0.0.1".to_string())
});

pub static DB_USERNAME: Lazy<String> = Lazy::new(|| {
    env::var("DB_USERNAME")
        .ok()
        .unwrap_or("username".to_string())
});

pub static DB_PASSWORD: Lazy<String> = Lazy::new(|| {
    env::var("DB_PASSWORD")
        .ok()
        .unwrap_or("password".to_string())
});

pub static DB_HOST: Lazy<String> = Lazy::new(|| {
    env::var("DB_HOST")
        .ok()
        .unwrap_or("127.0.0.1".to_string())
});

pub static DB_PORT: Lazy<u16> = Lazy::new(||{
    env::var("DB_PORT")
        .ok()
        .and_then(|db_port: String| db_port.parse::<u16>().ok())
        .unwrap_or(5432)
});

pub static DB_DATABASE_NAME: Lazy<String> = Lazy::new(|| {
    env::var("DB_DATABASE_NAME")
        .ok()
        .unwrap_or("database_name".to_string())
});


