/// HTTP specific worker, used to call HTTP/HTTPS urls
pub struct HttpWorker {
    pub url: String,
    pub count: u8,
    pub timeout: u8,
}

impl Default for HttpWorker {
    fn default() -> Self {
        Self {
            url: "undefined".to_string(),
            count: 1,
            timeout: 5,
        }
    }
}

pub trait HttpFlow {
    fn connect(&self) -> Result<u64, String>;
}

impl HttpFlow for HttpWorker {
    fn connect(&self) -> Result<u64, String> {
        let error = format!(
            "Not implemented.\n\tURL: {}\n\tcount: {}\n\ttimeout: {}",
            self.url, self.count, self.timeout
        );
        Err(error)
    }
}
