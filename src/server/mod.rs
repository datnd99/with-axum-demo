use std::net::SocketAddr;

use state::AppState;
use tokio::net::TcpListener;

use crate::{configure::AppConfig, error::AppResult, router::create_router_app};

pub mod state;

// tạo cấu trúc thể hiện cho server
pub struct AppServer {
    pub state: AppState,
    tcp: TcpListener,
}


// tạo phuong thức thực thi cho server
impl AppServer {
    pub async fn new(mut config: AppConfig) -> AppResult<Self> {
        let tcp: TcpListener = TcpListener::bind(config.server.get_socket_addr()?).await?;
        let addr: SocketAddr = tcp.local_addr()?;

        config.server.port = addr.port();
        let state: AppState = AppState::new(config).await?;

        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        let router = create_router_app(self.state);
        axum::serve(self.tcp, router).await?;

        Ok(())
    }
}
