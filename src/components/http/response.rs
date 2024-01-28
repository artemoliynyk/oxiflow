//! HTTP client response representation

use std::time::Instant;

use reqwest::Response;

pub struct HttpResponse {
    /// request URL
    pub url: String,

    /// request method
    pub method: String,

    /// HTTP response code
    pub code: u16,

    /// response time from the server in ms
    pub response_time: u128,

    /// request start time
    pub time: Instant,
}
impl HttpResponse {
    /// Create new response
    pub fn new(res: Response, method: String, elapsed: u128, time: Instant) -> HttpResponse {
        HttpResponse {
            url: res.url().to_string(),
            method,
            code: res.status().as_u16(),
            response_time: elapsed,
            time,
        }
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            url: Default::default(),
            method: Default::default(),
            code: Default::default(),
            response_time: Default::default(),
            time: Instant::now(),
        }
    }
}

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Success: Status – {}, Response time - {} ms",
            self.code, self.response_time
        )
    }
}
