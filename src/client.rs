use reqwest::{Client, Error, Response};
use std::sync::Arc;
#[derive(Clone)]
pub struct SenderClient {
    pub client: Arc<Client>,
    pub id: String,
    pub url: String,
}

impl SenderClient {
    pub fn new(id: &str, url: &str) -> SenderClient {
        Self {
            client: Arc::new(Client::new()),
            id: id.to_string(),
            url: url.to_string(),
        }
    }

    // Asynchronous GET request
    pub async fn get_read_request(&self, endpoint: &str) -> Result<Response, Error> {
        let full_url = format!("{}/{}", self.url, endpoint);
        self.client.get(full_url).send().await
    }

    // Asynchronous POST request
    pub async fn post_write_request(
        &self,
        endpoint: &str,
        body: String,
    ) -> Result<Response, Error> {
        let full_url = format!("{}/{}", self.url, endpoint);
        self.client.post(full_url).body(body).send().await
    }
}
