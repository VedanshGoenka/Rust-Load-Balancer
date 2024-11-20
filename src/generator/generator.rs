#[tokio::main]
use clap::Parser;

// Command Line Args
#[derive(Parser, Debug)]
#[command(name = "Request Generator")]
struct Args {
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
struct Generator {
    client_list: Vec<SenderClient>,
    url: String,
}

impl Generator {
    pub fn new(url: &str, num_clients: usize) -> Generator {
        let mut client_list = Vec::new();
        for i in 0..num_clients {
            // TODO: Does this i conversion work?
            client_list.push(SenderClient::new(i.to_string(), url));
        }

        Self {
            client_list: client_list,
            url: url.to_string(),
        }
    }

    pub async fn run(&self, read_write_ratio: f64) {
        let tasks: Vec<_> = self
            .client_list
            .iter()
            .map(|client| {
                let url = self.url.clone();
                task::spawn(async move {
                    // Determine if Read or Write
                    let is_read = rand::random::<f64>() < read_write_ratio;

                    if is_read {
                        match client.get_read_request(&url).await {
                            Ok(response) => {
                                println!("Client {} Read Response: {}", client.id, response)
                            }
                            Err(e) => eprintln!("Client {} Read Failed: {}", client.id, e),
                        }
                    } else {
                        let body = format!("Client {} sending this Write with body", client.id);
                        match client.post_write_request(&url, body).await {
                            Ok(response) => {
                                println!("Client {} Write Response: {}", client.id, response)
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
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let generator = Generator::new(&args.url, args.num_clients);
    generator.run(args.read_write_ratio).await;
}
