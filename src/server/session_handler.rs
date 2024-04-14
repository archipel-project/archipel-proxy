use std::net::SocketAddr;

use tokio::{io, net::TcpStream};
use tracing::debug;

use super::{config::Config, server_data::SharedServerData};

#[derive(Debug)]
pub struct SessionHandler {
    pub config: Config,
}

impl SessionHandler {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn handle_connection(
        &self,
        client_stream: TcpStream,
        remote_addr: SocketAddr,
        server_data: SharedServerData,
    ) -> anyhow::Result<()> {
        debug!("Handling connection from {}", remote_addr);

        client_stream.set_nodelay(true)?;

        let server_stream: TcpStream = {
            let data = server_data.lock().await;
            TcpStream::connect(data.server_addr).await?
        };

        let (mut client_reader, mut client_writer) = tokio::io::split(client_stream);
        let (mut server_reader, mut server_writer) = tokio::io::split(server_stream);

        let addr_clone = remote_addr.clone();
        tokio::spawn(async move {
            let result = io::copy(&mut client_reader, &mut server_writer).await;
            if let Some(err) = result.err() {
                debug!(
                    "{}: An error occurred in client-to-server bridge. Maybe disconnected: {}",
                    addr_clone, err
                );
            }
        });
        let result = tokio::io::copy(&mut server_reader, &mut client_writer).await;
        if let Some(err) = result.err() {
            debug!(
                "{}: An error occurred in server-to-client bridge. Maybe disconnected: {}",
                addr_clone, err
            );
        }
        Ok(())
    }
}
