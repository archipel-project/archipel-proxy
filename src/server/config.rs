use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use tokio::fs;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(default)]
pub struct Config {
    /// The address to bind the server to.
    /// TODO: add a validator for the address
    #[validate(length(min = 1, message = "The bind address must not be empty"))]
    pub(crate) bind: String,

    /// Message Of The Day, shown to the client when they connect.
    pub(crate) motd: String,

    /// The maximum number of players that are displayed in the server list.
    #[validate(range(
        min = 1,
        message = "The maximum number of players must be greater than 0"
    ))]
    pub(crate) show_max_players: u32,

    /// If true, the server will allow only online-mode clients to connect.
    pub(crate) online_mode: bool,

    /// Servers configurations
    #[serde(flatten)]
    pub(crate) servers: ServersConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:25577".into(),
            motd: "<orange>An Archipel Proxy server".into(),
            online_mode: false,
            show_max_players: 500,
            servers: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ServersConfig {
    /// Association between the server name and the server address.
    #[validate(length(min = 1, message = "At least one server must be defined"))]
    pub servers: HashMap<String, String>,

    /// The order in which the server will attempt to connect to the servers.
    #[validate(length(min = 1, message = "At least one server must be defined"))]
    pub attempt_connection_order: Vec<String>,
}

impl Default for ServersConfig {
    fn default() -> Self {
        Self {
            servers: HashMap::new(),
            attempt_connection_order: Vec::new(),
        }
    }
}

impl Iterator for ServersConfig {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, address) = self.servers.iter().next()?;
        Some((name.clone(), address.clone()))
    }
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut config = Config::default();

        // Ensure the configuration file exists, otherwise we just create it with the default values.
        if !path.as_ref().exists() {
            let str_config = toml::to_string(&config)?;
            fs::write(&path, str_config).await?;
        }

        let str_config = fs::read_to_string(path).await?;
        config = toml::from_str(&str_config)?;

        Ok(config)
    }
}
