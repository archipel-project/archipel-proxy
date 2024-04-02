use tracing::info;
use validator::Validate;

mod config;

#[derive(Debug)]
pub struct ProxyServer {}

impl ProxyServer {
    pub fn new() -> Self {
        ProxyServer {}
    }

    pub async fn start(self) -> anyhow::Result<()> {
        info!("Booting up the server...");

        // Load the configuration file
        let config = config::Config::load("proxy.toml").await?;

        config.validate()?;

        Ok(())
    }
}
