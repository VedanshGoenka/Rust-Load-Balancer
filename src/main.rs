//! Main entry point for the load balancer application
use clap::Parser;
use rust_load_balancer::generator::{Generator, GeneratorArgs};
use rust_load_balancer::server::{Server, ServerArgs};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(name = "Rust Load Balancer")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Start a server instance
    Server(ServerArgs),
    /// Start a generator instance
    Generator(GeneratorArgs),
    /// Start both server and generator
    Both {
        #[command(flatten)]
        server: ServerArgs,
        #[command(flatten)]
        generator: GeneratorArgs,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Server(server_args) => {
            // Run server only
            let server = Server::new(
                server_args.port,
                server_args.get_delay,
                server_args.post_delay,
            );
            println!("Starting server on port {}...", server_args.port);
            server.run().await;
        }
        Command::Generator(generator_args) => {
            // Run generator only
            let generator = Generator::new(&generator_args.url, generator_args.num_clients);
            println!("Starting generator...");
            std::sync::Arc::new(generator)
                .run(generator_args.read_write_ratio)
                .await;
        }
        Command::Both {
            server: server_args,
            generator: generator_args,
        } => {
            // Run both server and generator
            let server = Server::new(
                server_args.port,
                server_args.get_delay,
                server_args.post_delay,
            );
            let generator = Generator::new(&generator_args.url, generator_args.num_clients);
            let generator = std::sync::Arc::new(generator);

            // Spawn server task
            let server_handle = tokio::spawn(async move {
                println!("Starting server on port {}...", server_args.port);
                server.run().await;
            });

            // Wait for server to start
            sleep(Duration::from_secs(2)).await;

            // Spawn generator task
            let generator_handle = tokio::spawn({
                let generator = generator.clone();
                async move {
                    println!("Starting generator...");
                    generator.run(generator_args.read_write_ratio).await;
                }
            });

            // Wait for both tasks
            tokio::try_join!(server_handle, generator_handle)?;
        }
    }

    Ok(())
}
