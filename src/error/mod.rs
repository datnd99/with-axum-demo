pub type AppResult<T = ()> = std::result::Result<T, AppError>;

// #[derive(Debug, thiserror::Error, ToSchema)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0} parse error")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("{0} input/output error")]
    IoError(#[from] std::io::Error),
}
