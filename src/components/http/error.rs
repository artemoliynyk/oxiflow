//! Error HTTP client representation

use std::time::Instant;

pub struct HttpError {
    /// request URL
    pub url: String,

    /// request method
    pub method: String,

    // error message
    error: String,

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u128>,

    /// request start time
    pub time: Instant,
}

impl Default for HttpError {
    fn default() -> Self {
        Self {
            url: Default::default(),
            method: Default::default(),
            error: Default::default(),
            timeout: Default::default(),
            time: Instant::now(),
        }
    }
}

impl HttpError {
    /// Create new Error and resolve timeout from elapsed time if it happened
    pub fn new(err: reqwest::Error, method: String, elapsed: u128, time: Instant) -> HttpError {
        let timeout_info = match err.is_timeout() {
            true => Some(elapsed),
            false => None,
        };

        HttpError {
            url: err.url().unwrap().to_string(),
            method,
            error: err.to_string(),
            timeout: timeout_info,
            time
        }
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Timeout - {}",
            self.error,
            self.timeout.map_or("No".to_string(), |time_ms| {
                format!(", {} (ms)", time_ms)
            })
        )
    }
}
