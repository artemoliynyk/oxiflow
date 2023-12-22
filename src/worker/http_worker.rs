use std::time::Duration;

use reqwest::{Client, RequestBuilder, ClientBuilder};


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
}
