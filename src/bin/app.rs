use dotenvy::dotenv;
use expense_service::{configure::{self, AppConfig}, error::AppResult, server::AppServer};
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    
    let _file_appender_guard: WorkerGuard = configure::tracing::init()?;
    info!("Tải biến môi trường từ file vào hệ thống thành công");
    info!("Khởi tạo theo dõi lỗi cho hệ thống thành công");

    let config: AppConfig = AppConfig::read();
    info!("Đọc config của cho hệ thống thành công");


    let server: AppServer = AppServer::new(config).await?;
    info!("Tạo server cho hệ thống thành công");

    server.run().await?;
    info!("Khởi chạy hệ thống thành công");

    Ok(())
}
