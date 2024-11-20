use tokio;

struct SenderClient {
    client: std::sync::Arc<reqwest::Client>,
    id: String,
    url: String,
}

impl SenderClient {
    pub fn new(id: &str, url: &str) -> SenderClient {
        Self {
            client: std::sync::Arc(reqwest::Client::new()),
            id: id.to_string(),
            url: url.to_string(),
        }
    }

    // Stub Get/Post Requests
    pub fn get_read_request(endpoint: &str) -> Result<Response> {
        reqwest::get(url + "/" + endpoint)
    }

    pub fn post_write_request(endpoint: &str, body: String) -> Result<Response> {
        reqwest::post(url + "/" + endpoint).body(body).send()
    }
}
