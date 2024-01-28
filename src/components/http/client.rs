//! HTTP client to perform the requests

use std::{
    error::Error,
    time::{Duration, Instant},
};

use super::{error::HttpError, response::HttpResponse, HttpResult};
use reqwest::{Client, ClientBuilder, Request, RequestBuilder};

/// supported HTTP-methods, used for the command line args filtering
pub const SUPPORTED_METHODS: [&str; 5] = ["GET", "POST", "DELETE", "PUT", "PATCH"];

/// check if method arg passed from the command line is valid and supported
pub fn is_supported_method(method: &str) -> bool {
    SUPPORTED_METHODS.contains(&method.trim().to_uppercase().as_str())
}

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

    pub fn resolve_request(&self, method: String, url: String) -> Result<Request, Box<dyn Error>> {
        let method_upper = method.trim().to_uppercase();

        if !is_supported_method(&method) {
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
