use std::{
    error::Error,
    time::{Duration, Instant},
};

use reqwest::{Client, ClientBuilder, RequestBuilder};

use crate::worker;

use super::{error::ClientError, response::ClientResponse};

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

    pub fn patch(&self, url: String) -> RequestBuilder {
        self.client.patch(url)
    }

    pub fn delete(&self, url: String) -> RequestBuilder {
        self.client.delete(url)
    }

    pub fn resolve_request(
        &self,
        method: String,
        url: String,
    ) -> Result<RequestBuilder, Box<dyn Error>> {
        let method_upper = method.trim().to_uppercase();

        if !worker::is_supported_method(&method) {
            return Err(format!("Unsupported method: '{}'", &method).into());
        }

        let req = match method_upper.as_str() {
            "GET" => self.get(url),
            "POST" => self.post(url),
            "PUT" => self.put(url),
            "PATCH" => self.patch(url),
            "DELETE" => self.delete(url),
            _ => panic!("Unmatched method found, previous checks failed. This is a bug!"),
        };

        Ok(req)
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
