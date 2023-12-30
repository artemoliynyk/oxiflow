use std::time::{Duration, Instant};

use reqwest::{Client, ClientBuilder, RequestBuilder};

use super::{response::ClientResponse, error::ClientError};

pub type ClientResult = std::result::Result<ClientResponse, ClientError>;

/// HTTP specific worker, used to call HTTP/HTTPS urls
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(timeout_sec: u8) -> HttpClient {
        let default_timeout = Duration::from_secs(timeout_sec as u64);

        HttpClient {
            client: ClientBuilder::new()
                .timeout(default_timeout)
                .build()
                .expect("Error creating client"),
        }
    }

    pub fn get(&self, url: String) -> RequestBuilder {
        self.client.get(url)
    }

    pub fn post(&self, url: String) -> RequestBuilder {
        self.client.post(url)
    }

    pub fn put(&self, url: String) -> RequestBuilder {
        self.client.put(url)
    }

    pub fn delete(&self, url: String) -> RequestBuilder {
        self.client.delete(url)
    }
}


pub async fn execute_request(request: RequestBuilder) -> ClientResult {
    let start = Instant::now();
    let response = request.send().await;
    let elapsed = start.elapsed().as_millis();

    match response {
        Ok(res) => Ok(ClientResponse::new(res, elapsed)),
        Err(err) => Err(ClientError::new(err, elapsed)),
    }
}