//! Main entry point for the load balancer application
use clap::Parser;
use rust_load_balancer::generator::{Generator, GeneratorArgs};
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Some issues with Crate Dependencies
    let args = GeneratorArgs::parse();
    let generator = Generator::new(&args.url, args.num_clients);
    println!("Beginning Generator...");
    std::sync::Arc::new(generator)
        .run(args.read_write_ratio)
        .await;
    Ok(())
}
