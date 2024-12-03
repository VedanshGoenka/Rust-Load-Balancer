use crate::client::SenderClient;
use clap::Parser;
use futures::future::join_all;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "Generator")]
pub struct GeneratorArgs {
    #[arg(short = 'u', long, default_value = "http://127.0.0.1:8000")]
    pub url: String,

    #[arg(short = 'n', long, default_value = "10")]
    pub num_requests: usize,

    #[arg(short = 'c', long, default_value = "5")]
    pub concurrent_clients: usize,

    #[arg(short = 'r', long, default_value = "0.7")]
    pub get_ratio: f64,
}

pub struct Generator {
    url: String,
    num_clients: usize,
    get_ratio: f64,
}

impl Generator {
    pub fn new(url: &str, num_clients: usize, get_ratio: f64) -> Self {
        Self {
            url: url.to_string(),
            num_clients,
            get_ratio,
        }
    }

    pub async fn run(&self, num_requests: usize) {
        let successful_requests = Arc::new(AtomicUsize::new(0));

        println!(
            "Starting load test with {} clients, {} total requests ({:.0}% GET, {:.0}% POST)",
            self.num_clients,
            num_requests,
            self.get_ratio * 100.0,
            (1.0 - self.get_ratio) * 100.0
        );

        let start_time = Instant::now();
        let requests_per_client = num_requests / self.num_clients;
        let mut all_futures = Vec::new();

        // Create all request futures upfront
        for i in 0..self.num_clients {
            let successful_requests = Arc::clone(&successful_requests);
            let client = SenderClient::new(&i.to_string(), &self.url);

            for j in 0..requests_per_client {
                let successful_requests = Arc::clone(&successful_requests);
                let is_get = (j as f64 / requests_per_client as f64) < self.get_ratio;
                let client = client.clone();

                let future = tokio::spawn(async move {
                    let result = if is_get {
                        client.get_read_request("").await
                    } else {
                        client.post_write_request("", format!("test{}", i)).await
                    };

                    match result {
                        Ok(_) => {
                            successful_requests.fetch_add(1, Ordering::Relaxed);
                            println!(
                                "Client {} - {} request {} successful",
                                i,
                                if is_get { "GET" } else { "POST" },
                                j
                            )
                        }
                        Err(e) => eprintln!(
                            "Client {} - {} request {} failed: {}",
                            i,
                            if is_get { "GET" } else { "POST" },
                            j,
                            e
                        ),
                    }
                });
                all_futures.push(future);
            }
        }

        // Run all requests concurrently
        join_all(all_futures).await;

        let duration = start_time.elapsed();
        let successful = successful_requests.load(Ordering::Relaxed);
        println!("Load test completed in {:?}", duration);
        println!(
            "Successful requests: {}/{} ({:.1}%)",
            successful,
            num_requests,
            (successful as f64 / num_requests as f64) * 100.0
        );
        println!(
            "Average request rate: {:.2} requests/second",
            successful as f64 / duration.as_secs_f64()
        );
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let args = GeneratorArgs::parse();
    let generator = Generator::new(&args.url, args.concurrent_clients, args.get_ratio);
    generator.run(args.num_requests).await;
}
