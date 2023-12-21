use std::time::{Duration, Instant};

use reqwest::{Client, ClientBuilder, RequestBuilder};

use super::{WorkerError, WorkerResponse, WorkerResult};

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

    pub async fn execute(self, request: RequestBuilder) -> WorkerResult {
        let start = Instant::now();
        let response = request.send().await;
        let elapsed = start.elapsed().as_millis();

        match response {
            Ok(res) => Ok(WorkerResponse {
                code: res.status().as_u16(),
                response_time: elapsed,
            }),
            Err(err) => {
                let timeout_info = match err.is_timeout() {
                    true => Some(elapsed),
                    false => None,
                };

                Err(WorkerError {
                    error: err.to_string(),
                    is_timeout: err.is_timeout(),
                    timeout: timeout_info,
                })
            }
        }
    }
}
