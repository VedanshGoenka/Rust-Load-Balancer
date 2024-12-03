use reqwest::{Client, Error, Response};
use std::sync::Arc;
use tokio::time::Duration;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 100;

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

    async fn retry_request<F, Fut>(retries: u32, f: F) -> Result<Response, Error>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<Response, Error>>,
    {
        let mut attempt = 0;
        loop {
            match f().await {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    attempt += 1;
                    if attempt >= retries {
                        return Err(e);
                    }
                    tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        }
    }

    pub async fn get_read_request(&self, endpoint: &str) -> Result<Response, Error> {
        let full_url = format!("{}/{}", self.url, endpoint);
        let client = self.client.clone();
        Self::retry_request(MAX_RETRIES, || {
            client.get(&full_url).header("Connection", "close").send()
        })
        .await
    }

    pub async fn post_write_request(
        &self,
        endpoint: &str,
        body: String,
    ) -> Result<Response, Error> {
        let full_url = format!("{}/{}", self.url, endpoint);
        let client = self.client.clone();
        Self::retry_request(MAX_RETRIES, || {
            client
                .post(&full_url)
                .header("Connection", "close")
                .body(body.clone())
                .send()
        })
        .await
    }
}
