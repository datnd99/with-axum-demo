use serde::Deserialize;

use crate::constant::{ENV_VARS};

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
}

impl DatabaseConfig {
  pub fn create_url () -> String {
    ENV_VARS.database_url.clone()
  }
}
