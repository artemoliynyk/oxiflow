//! HTTP client to perform the requests

use std::{
    error::Error,
    time::{Duration, Instant},
};

use reqwest::{Client, ClientBuilder, RequestBuilder};

use crate::components::worker;

use super::{error::HttpError, response::HttpResponse};

pub type HttpResult = std::result::Result<HttpResponse, HttpError>;

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

pub async fn execute_request(request: RequestBuilder) -> HttpResult {
    let start = Instant::now();

    // TODO: rewrite this abomination
    let method = request
        .try_clone()
        .unwrap()
        .build()
        .unwrap()
        .method()
        .to_string();

    let response = request.send().await;
    let elapsed = start.elapsed().as_millis();

    match response {
        Ok(res) => Ok(HttpResponse::new(res, method, elapsed)),
        Err(err) => Err(HttpError::new(err, method, elapsed)),
    }
}
