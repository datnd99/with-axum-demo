use billapi::{configure::AppConfig, error::AppResult, server::AppServer};

#[tokio::main]
async fn main() -> AppResult<()> {

    let config = AppConfig::read();

    let server: AppServer = AppServer::new(config).await?;
    
    server.run().await?;

    Ok(())
}
