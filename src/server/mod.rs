use clap::Parser;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::{sleep, Duration},
};

#[derive(Parser, Debug)]
#[command(name = "Server")]
pub struct ServerArgs {
    // Port to listen on
    #[arg(short = 'P', long, default_value = "8000")]
    pub port: u16,

    // Delay for GET requests in milliseconds
    #[arg(short = 'g', long, default_value = "1000")]
    pub get_delay: u64,

    // Delay for POST requests in milliseconds
    #[arg(short = 'p', long, default_value = "500")]
    pub post_delay: u64,
}

pub struct Server {
    port: u16,
    get_delay: u64,
    post_delay: u64,
}

impl Server {
    pub fn new(port: u16, get_delay: u64, post_delay: u64) -> Self {
        Self {
            port,
            get_delay,
            post_delay,
        }
    }

    pub async fn run(&self) {
        // Bind to localhost
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server listening on {}", addr);

        loop {
            // Accept connection
            let (socket, _) = listener.accept().await.unwrap();
            let get_delay = self.get_delay;
            let post_delay = self.post_delay;

            // Spawn new task to handle connection
            tokio::spawn(async move {
                Self::handle_connection(socket, get_delay, post_delay).await;
            });
        }
    }

    async fn handle_connection(mut socket: TcpStream, get_delay: u64, post_delay: u64) {
        // Buffer to read request from socket
        let mut buffer = [0; 1024];

        // Read request from socket
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        // Convert buffer to string
        let request = String::from_utf8_lossy(&buffer[..n]);

        // Get first line of request
        let first_line = request.lines().next().unwrap_or("");
        let (method, _) = first_line.split_once(' ').unwrap_or(("", ""));

        // Sleep for delay based on method
        match method {
            "GET" => sleep(Duration::from_millis(get_delay)).await,
            "POST" => sleep(Duration::from_millis(post_delay)).await,
            _ => {}
        }

        // Response message
        let msg = format!("Request Received of type: {}", method);
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
            msg.len(),
            msg
        );

        // Write response and shutdown
        if let Ok(()) = socket.write_all(response.as_bytes()).await {
            let _ = socket.shutdown().await;
        }
        drop(socket);
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let args = ServerArgs::parse();
    let server = Server::new(args.port, args.get_delay, args.post_delay);
    server.run().await;
}
