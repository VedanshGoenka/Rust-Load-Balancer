//! Main entry point for the load balancer application
use clap::Parser;
use rust_load_balancer::balancer::LoadBalancer;
use rust_load_balancer::server::Server;
use rust_load_balancer::generator::{Generator, GeneratorArgs};

#[derive(Parser, Debug)]
#[command(name = "Rust Load Balancer")]
enum Command {
    #[command(name = "balancer")]
    Balancer {
        #[arg(short = 'p', long, default_value = "8000")]
        port: u16,

        #[arg(short = 's', long = "servers", value_delimiter = ',')]
        servers: Vec<String>,
    },
    #[command(name = "server")]
    Server {
        #[arg(short = 'p', long, default_value = "8001")]
        port: u16,

        #[arg(short = 'g', long, default_value = "100")]
        get_delay: u64,

        #[arg(short = 'o', long, default_value = "200")]
        post_delay: u64,
    },
    #[command(name = "generator")]
    Generator {
        #[command(flatten)]
        args: GeneratorArgs,
    },
}

#[tokio::main]
async fn main() {
    match Command::parse() {
        Command::Balancer { port, servers } => {
            println!("Starting load balancer on port {} with servers: {:?}", port, servers);
            let balancer = LoadBalancer::new(port, servers);
            balancer.run().await;
        }
        Command::Server { port, get_delay, post_delay } => {
            println!("Starting server on port {} (GET delay: {}ms, POST delay: {}ms)", 
                port, get_delay, post_delay);
            let server = Server::new(port, get_delay, post_delay);
            server.run().await;
        }
        Command::Generator { args } => {
            println!("Starting load generator");
            let generator = Generator::new(&args.url, args.concurrent_clients, args.get_ratio);
            generator.run(args.num_requests).await;
        }
    }
}
