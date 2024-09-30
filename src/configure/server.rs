use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
  pub addr: String,
  pub port: u16,
}

impl ServerConfig {
  pub fn get_addr(&self) -> String {
    format!("{}:{}", self.addr, self.port)
  }

  pub fn get_http_addr(&self) -> String {
    format!("http://{}:{}", self.addr, self.port)
  }
  pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
    let addr: String = self.get_addr();

    addr.parse()
  }
}