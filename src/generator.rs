use crate::client::SenderClient;
use clap::Parser;
use tokio::task;

#[derive(Parser, Debug)]
#[command(name = "Request Generator")]
pub struct GeneratorArgs {
    // url of Load Balancer/Server to send requests to
    #[arg(short = 'u', long)]
    pub url: String,

    // Number of SenderClients
    #[arg(short = 'n', long, default_value = "10")]
    pub num_clients: usize,

    // Ratio of Reads to Write Requests
    #[arg(short = 'r', long, default_value = "0.5")]
    pub read_write_ratio: f64,
}

/**
 * Creates a multi-threaded Generator to send requests in parallel for every SenderClient
 */
pub struct Generator {
    client_list: Vec<SenderClient>,
    url: String
}

impl Generator {
    pub fn new(url: &str, num_clients: usize) -> Generator {
        let mut client_list = Vec::new();
        for i in 0..num_clients {
            client_list.push(SenderClient::new(&i.to_string(), url));
        }

        Self {
            client_list,
            url: url.to_string()
        }
    }

    pub async fn run(self: std::sync::Arc<Self>, read_write_ratio: f64) {
        // Start overall timing
        let total_start = std::time::Instant::now();

        let tasks: Vec<_> = self
            .client_list
            .iter()
            .map(|client| {
                let client = client.clone();
                let url = self.url.clone();
                task::spawn(async move {
                    // Determine if Read or Write
                    let is_read = rand::random::<f64>() < read_write_ratio;

                    // Start timing
                    let start_time = std::time::Instant::now();

                    if is_read {
                        match client.get_read_request(&url).await {
                            Ok(_response) => {
                                let duration = start_time.elapsed();
                                println!(
                                    "Client {} Read Response - Time: {:.2}ms",
                                    client.id,
                                    duration.as_secs_f64() * 1000.0
                                )
                            }
                            Err(e) => eprintln!("Client {} Read Failed: {}", client.id, e),
                        }
                    } else {
                        let body = format!("Client {} sending this Write with body", client.id);
                        match client.post_write_request(&url, body).await {
                            Ok(_response) => {
                                let duration = start_time.elapsed();
                                println!(
                                    "Client {} Write Response - Time: {:.2}ms",
                                    client.id,
                                    duration.as_secs_f64() * 1000.0
                                )
                            }
                            Err(e) => eprintln!("Client {} Write Failed: {}", client.id, e),
                        }
                    }
                })
            })
            .collect();

        for task in tasks {
            let _ = task.await;
        }

        // Calculate and print overall time
        let total_elapsed = total_start.elapsed();
        println!("Finished all tasks!");
        println!(
            "Total time for all requests: {:.2}ms", 
            total_elapsed.as_secs_f64() * 1000.0
        );
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let args = GeneratorArgs::parse();
    let generator = Generator::new(&args.url, args.num_clients);
    println!("Beginning Generator...");
    std::sync::Arc::new(generator)
        .run(args.read_write_ratio)
        .await;
}
