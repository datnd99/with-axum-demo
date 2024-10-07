use tracing::{subscriber, Subscriber};
use tracing_appender::{
  non_blocking::WorkerGuard, // Sử dụng WorkerGuard để quản lý log không đồng bộ
  rolling::{RollingFileAppender, Rotation}, // Cấu hình cho log file có thể lăn theo thời gian
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer}; // Sử dụng formatter Bunyan (định dạng JSON)
use tracing_log::LogTracer; // Tracer để chuyểng đổi log từ thư viện `log` sang `tracing`
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry}; // SubscriberExt giúp nối nhiều Layer cho tracing

use crate::error::AppResult; // Định nghĩa kết quả ứng dụng từ module `error`

/// Tạo một subscriber để cấu hình hệ thống log
/// 
/// `name`: tên của layer log, ví dụ "app".
/// `env_filter`: bộ lọc môi trường giúp điều chỉnh mức độ log thông qua biến môi trường.
/// `writer`: đầu ra của log, ví dụ như stdout hay file.
///
/// Trả về một subscriber có thể đồng bộ và chia sẻ giữa các luồng.
fn create_subscriber<W>(
  name: &str, 
  env_filter: EnvFilter, 
  writer: W,
) -> impl Subscriber + Sync + Send 
where
  W: for<'a> MakeWriter<'a> + Send + Sync + 'static, 
{
  // `Registry::default` tạo một registry mặc định cho hệ thống log
  Registry::default()
    .with(env_filter) // Áp dụng bộ lọc môi trường
    .with(JsonStorageLayer) // Tạo layer lưu trữ dưới dạng JSON
    .with(BunyanFormattingLayer::new(name.into(), std::io::stdout)) // Format log bằng Bunyan và xuất ra stdout
    .with(BunyanFormattingLayer::new(name.into(), writer)) // Format log Bunyan cho writer (ví dụ như file)
}

/// Khởi tạo global subscriber cho hệ thống log
///
/// `subscriber`: đối tượng subscriber đã được cấu hình.
///
/// Trả về `Result` để báo cáo thành công hoặc lỗi nếu không thể thiết lập global subscriber.
pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
where
  S: Subscriber + Send + Sync + 'static, 
{
  // Chuyển đổi log từ `log` sang `tracing`
  LogTracer::init()?;
  // Đặt subscriber toàn cục cho toàn bộ hệ thống log
  subscriber::set_global_default(subscriber)?;
  Ok(())
}

/// Hàm khởi tạo logging subsystem cho ứng dụng
///
/// Tạo một file logger với rotation (theo ngày), và khởi tạo hệ thống logging với subscriber.
///
/// Trả về một `WorkerGuard` để đảm bảo rằng log sẽ không bị mất khi chương trình kết thúc.
pub fn init() -> AppResult<WorkerGuard> {
  // Tạo rolling file appender, tạo log mới mỗi ngày trong thư mục "logs" với tên "app.log"
  let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
  
  // Tạo appender không đồng bộ để tránh blocking hệ thống khi ghi log
  let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
  
  // Gọi hàm `init_subscriber` để khởi tạo global subscriber cho logging
  init_subscriber(create_subscriber(
    "app", // Tên của hệ thống log (ví dụ: "app")
    EnvFilter::from_default_env(), // Bộ lọc log dựa trên biến môi trường
    // EnvFilter::new("info"),
    file_appender, // Appender cho log file
  ))?;
  
  // Trả về guard để đảm bảo file appender được giữ sống suốt thời gian chạy của ứng dụng
  Ok(file_appender_guard)
}
