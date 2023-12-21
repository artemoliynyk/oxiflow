use std::{future::Future, time::Duration};

use reqwest::{Client, ClientBuilder, Response};

/// HTTP specific worker, used to call HTTP/HTTPS urls
pub struct HttpWorker {
    client: Client,
}

impl HttpWorker {
    pub fn new(timeout_sec: u8) -> HttpWorker {
        let default_timeout = Duration::from_secs(timeout_sec as u64);

        HttpWorker {
            client: ClientBuilder::new()
                .timeout(default_timeout)
                .build()
                .expect("Error creating client"),
        }
    }

    pub fn get(self, url: String) -> impl Future<Output = Result<Response, reqwest::Error>>{
        self.client.get(url).send()
    }

    pub fn post(self, url: String) -> impl Future<Output = Result<Response, reqwest::Error>>{
        self.client.post(url).send()
    }

    pub fn put(self, url: String) -> impl Future<Output = Result<Response, reqwest::Error>>{
        self.client.put(url).send()
    }

    pub fn delete(self, url: String) -> impl Future<Output = Result<Response, reqwest::Error>>{
        self.client.delete(url).send()
    }
}
