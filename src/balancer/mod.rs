use crate::algorithms::{Algorithm, LoadBalancingAlgorithm};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{RwLock, Semaphore},
    time::{interval, Duration},
    signal,
};

const MAX_CONNECTIONS: usize = 500;
const METRICS_INTERVAL: u64 = 5; // seconds

#[derive(Clone)]
pub struct LoadBalancer {
    port: u16,
    servers: Arc<RwLock<Vec<String>>>,
    algorithm: Algorithm,
    connection_limiter: Arc<Semaphore>,
}

impl LoadBalancer {
    pub fn new(port: u16, servers: Vec<String>, algorithm_type: &str) -> Self {
        Self {
            port,
            servers: Arc::new(RwLock::new(servers)),
            algorithm: Algorithm::new(algorithm_type, None),
            connection_limiter: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        }
    }

    async fn print_metrics(&self, prefix: &str) {
        let metrics = self.algorithm.get_metrics().await;
        if !metrics.is_empty() {
            println!("\n{}", prefix);
            for (server, metric) in metrics {
                println!("{}: {}", server, metric);
            }
        }
    }

    pub async fn run(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Load balancer listening on {}", addr);

        // Start metrics reporting
        let algorithm = self.algorithm.clone();
        let metrics_task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(METRICS_INTERVAL));
            loop {
                interval.tick().await;
                let metrics = algorithm.get_metrics().await;
                if !metrics.is_empty() {
                    println!("\nServer Metrics:");
                    for (server, metric) in metrics {
                        println!("{}: {}", server, metric);
                    }
                }
            }
        });

        // Handle shutdown signal
        let shutdown = signal::ctrl_c();
        tokio::pin!(shutdown);

        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    let (client, _) = accept_result.unwrap();
                    let servers = Arc::clone(&self.servers);
                    let algorithm = self.algorithm.clone();
                    let this = self.clone();
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

                        algorithm.connection_started(&server).await;
                        let result = this.forward_request(client, server.clone()).await;
                        algorithm.connection_ended(&server).await;

                        if let Err(e) = result {
                            eprintln!("Error forwarding request to {}: {}", server, e);
                        }

                        drop(permit);
                    });
                }
                _ = &mut shutdown => {
                    println!("\nShutdown signal received. Printing final metrics...");
                    self.print_metrics("Final Server Metrics:").await;
                    metrics_task.abort();
                    break;
                }
            }
        }

        println!("Load balancer shutting down.");
    }

    async fn forward_request(&self, mut client: TcpStream, server_addr: String) -> std::io::Result<()> {
        // Read the request first
        let mut buffer = [0; 1024];
        let n = client.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);
        
        // Check if it's a metrics request
        if request.contains("GET /metrics") {
            let metrics = self.algorithm.get_metrics().await;
            let mut response = String::new();
            for (server, metric) in metrics {
                response.push_str(&format!("{}: {}\n", server, metric));
            }
            
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response.len(),
                response
            );
            client.write_all(response.as_bytes()).await?;
            client.shutdown().await?;
            return Ok(());
        }

        // Regular request forwarding
        let mut server = TcpStream::connect(&server_addr).await?;
        server.write_all(&buffer[..n]).await?;

        let (mut client_reader, mut client_writer) = client.split();
        let (mut server_reader, mut server_writer) = server.split();

        let client_to_server = tokio::io::copy(&mut client_reader, &mut server_writer);
        let server_to_client = tokio::io::copy(&mut server_reader, &mut client_writer);

        let (_client_bytes, server_bytes) = match tokio::join!(client_to_server, server_to_client) {
            (Ok(c), Ok(s)) => (c, s),
            _ => return Ok(()),
        };

        if server_bytes > 0 {
            client.shutdown().await?;
            drop(client);
        }

        Ok(())
    }
}
