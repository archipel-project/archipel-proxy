use std::process;

use tracing::error;

mod bootstrap;
mod server;

#[tokio::main]
async fn main() {
    bootstrap::init().expect("Failed to initialize the logger");

    let server = server::ProxyServer::new();

    match server.start().await {
        Ok(_) => {}
        Err(e) => {
            error!("An error occurred: {:?}", e);
            process::exit(1);
        }
    }
}
