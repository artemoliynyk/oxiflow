//! HTTP client to perform the requests

use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::components::http;
use crate::components::worker::request::WorkerRequest;

use crate::components::http::{error::HttpError, response::HttpResponse, HttpResult};
use reqwest::{Client, ClientBuilder, Request, RequestBuilder};

/// HTTP specific worker, used to call HTTP/HTTPS urlsØ
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

    pub fn resolve_request(&self, req: &WorkerRequest) -> Result<Request, Box<dyn Error>> {
        if !http::method_supported(&req.method) {
            return Err(format!("Unsupported method: '{}'", &req.method).into());
        }

        let url = req.url.clone();
        let req = match req.method.trim().to_uppercase().as_str() {
            "GET" => self.get(url),
            "POST" => self.post(url),
            "PUT" => self.put(url),
            "PATCH" => self.patch(url),
            "DELETE" => self.delete(url),
            _ => panic!("Unmatched method found, previous checks failed. This is a bug!"),
        };

        Ok(req.build().unwrap())
    }

    pub async fn execute_request(&self, request: Request) -> HttpResult {
        let start = Instant::now();
        let method = request.method().to_string();

        let response = self.client.execute(request).await;
        let elapsed = start.elapsed().as_millis();

        match response {
            Ok(res) => Ok(HttpResponse::new(res, method, elapsed, start)),
            Err(err) => Err(HttpError::new(err, method, elapsed, start)),
        }
    }
}
