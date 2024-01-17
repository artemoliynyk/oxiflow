//! Error HTTP client representation

#[derive(Default)]
pub struct HttpError {
    /// request URL
    pub url: String,

    /// request method
    pub method: String,

    // error message
    error: String,
    
    // TODO: remove this, we have Option<T> below
    is_timeout: bool,

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u128>,
}

impl HttpError {
    /// Create new Error and resolve timeout from elapsed time if it happened
    pub fn new(err: reqwest::Error, method: String, elapsed: u128) -> HttpError {
        let timeout_info = match err.is_timeout() {
            true => Some(elapsed),
            false => None,
        };

        HttpError {
            url: err.url().unwrap().to_string(),
            method,
            error: err.to_string(),
            is_timeout: err.is_timeout(),
            timeout: timeout_info,
        }
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Timeout - {}{}",
            self.error,
            self.is_timeout,
            if self.is_timeout {
                format!(", {} (ms)", self.timeout.unwrap())
            } else {
                "".to_string()
            }
        )
    }
}
