use std::sync::Arc;

use tracing::info;
use validator::Validate;

use crate::server::{config, session_handler::SessionHandler};

use super::connection_manager::ConnectionManager;

#[derive(Debug)]
pub struct ProxyServer {
    cm: ConnectionManager,
}

impl ProxyServer {
    pub fn new() -> Self {
        ProxyServer {
            cm: ConnectionManager::new(),
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Booting up the server...");

        // Load the configuration file
        let config = config::Config::load("proxy.toml").await?;

        config.validate()?;

        self.cm.bind(config.bind.parse()?).await?;

        let session_handler = SessionHandler::new(config);
        self.cm.listen(Arc::new(session_handler)).await?;

        Ok(())
    }
}
