use crate::client::SenderClient;
use clap::Parser;
use tokio::task;
#[derive(Parser, Debug)]
#[command(name = "Request Generator")]
pub struct Args {
    // url of Load Balancer/Server to send requests to
    // Passed to each SenderClient
    #[arg(short, long)]
    url: String,

    // Number of SenderClients
    #[arg(short, long, default_value = "10")]
    num_clients: usize,

    // Ratio of Reads to Write Requests
    #[arg(short, long, default_value = "0.5")]
    read_write_ratio: f64,
}

/**
 * Creates a multi-threaded Generator to send requests in parallel for every SenderClient
 */
pub struct Generator {
    client_list: Vec<SenderClient>,
    url: String,
}

impl Generator {
    pub fn new(url: &str, num_clients: usize) -> Generator {
        let mut client_list = Vec::new();
        for i in 0..num_clients {
            client_list.push(SenderClient::new(&i.to_string(), url));
        }

        Self {
            client_list,
            url: url.to_string(),
        }
    }

    pub async fn run(self: std::sync::Arc<Self>, read_write_ratio: f64) {
        let tasks: Vec<_> = self
            .client_list
            .iter()
            .map(|client| {
                let client = client.clone();
                let url = self.url.clone();
                task::spawn(async move {
                    // Determine if Read or Write
                    let is_read = rand::random::<f64>() < read_write_ratio;

                    if is_read {
                        match client.get_read_request(&url).await {
                            Ok(response) => {
                                println!("Client {} Read Response", client.id)
                            }
                            Err(e) => eprintln!("Client {} Read Failed: {}", client.id, e),
                        }
                    } else {
                        let body = format!("Client {} sending this Write with body", client.id);
                        match client.post_write_request(&url, body).await {
                            Ok(response) => {
                                println!("Client {} Write Response", client.id)
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

        println!("Finished all tasks!");
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let generator = Generator::new(&args.url, args.num_clients);
    println!("Beginning Generator...");
    std::sync::Arc::new(generator)
        .run(args.read_write_ratio)
        .await;
}
