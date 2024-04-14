use std::{net::SocketAddr, sync::Arc};

use tokio::{net::TcpListener, process, sync::Mutex};
use tracing::{error, info};

use super::{
    server_data::{ServerData, SharedServerData},
    session_handler::SessionHandler,
};

#[derive(Debug)]
pub struct ConnectionManager {
    tcp_listener: Option<TcpListener>,
    server_data: Option<SharedServerData>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            tcp_listener: None,
            server_data: None,
        }
    }

    pub async fn bind(&mut self, addr: SocketAddr) -> anyhow::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        self.tcp_listener = Some(listener);

        Ok(())
    }

    pub async fn listen(&mut self, handler: Arc<SessionHandler>) -> anyhow::Result<()> {
        let listener = self.tcp_listener.as_ref().expect("Listener not bound");

        let server_addr = "127.0.0.1:25565".parse()?;
        self.server_data = Some(Arc::new(Mutex::new(ServerData::new(server_addr))));

        info!("Listening on {}", listener.local_addr()?);

        loop {
            match listener.accept().await {
                Ok((stream, remote_addr)) => {
                    info!("Accepted connection from {}", remote_addr);

                    let handler = handler.clone();
                    let data = self.server_data.clone().unwrap();
                    tokio::spawn(async move {
                        if let Err(e) = handler.handle_connection(stream, remote_addr, data).await {
                            error!("Failed to handle connection: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {:?}", e);
                }
            }
        }
    }
}
