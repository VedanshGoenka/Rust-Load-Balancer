use crate::algorithms::{LoadBalancingAlgorithm, RoundRobin};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{RwLock, Semaphore},
};

const MAX_CONNECTIONS: usize = 200;

pub struct LoadBalancer {
    port: u16,
    servers: Arc<RwLock<Vec<String>>>,
    algorithm: RoundRobin,
    connection_limiter: Arc<Semaphore>,
}

impl LoadBalancer {
    pub fn new(port: u16, servers: Vec<String>) -> Self {
        Self {
            port,
            servers: Arc::new(RwLock::new(servers)),
            algorithm: RoundRobin::new(),
            connection_limiter: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        }
    }

    pub async fn run(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Load balancer listening on {}", addr);

        loop {
            let (client, _) = listener.accept().await.unwrap();
            let servers = Arc::clone(&self.servers);
            let algorithm = self.algorithm.clone();
            let permit = Arc::clone(&self.connection_limiter)
                .acquire_owned()
                .await
                .unwrap();

            tokio::spawn(async move {
                let server = {
                    let servers = servers.read().await;
                    match algorithm.next_server(&servers).await {
                        Some(server) => server,
                        None => return,
                    }
                };

                let _ = Self::forward_request(client, server).await;
                drop(permit);
            });
        }
    }

    async fn forward_request(mut client: TcpStream, server_addr: String) -> std::io::Result<()> {
        let mut server = TcpStream::connect(&server_addr).await?;
        tokio::io::copy_bidirectional(&mut client, &mut server).await?;
        Ok(())
    }
}
