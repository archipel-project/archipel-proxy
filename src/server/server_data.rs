use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct ServerData {
    pub total_users: u32,
    pub users_count: u32,
    pub server_addr: SocketAddr,
}

impl ServerData {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            total_users: 20,
            users_count: 0,
            server_addr: addr,
        }
    }
}

pub type SharedServerData = Arc<Mutex<ServerData>>;
