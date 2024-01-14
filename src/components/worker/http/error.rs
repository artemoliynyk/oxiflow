//! Error HTTP client representation

#[derive(Debug, Default)]
pub struct ClientError {
    error: String,
    is_timeout: bool,

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u128>,
}

impl ClientError {
    /// Create new Error and resolve timeout from elapsed time if it happened
    pub fn new(err: reqwest::Error, elapsed: u128) -> ClientError {
        let timeout_info = match err.is_timeout() {
            true => Some(elapsed),
            false => None,
        };

        ClientError {
            error: err.to_string(),
            is_timeout: err.is_timeout(),
            timeout: timeout_info,
        }
    }
}

impl std::fmt::Display for ClientError {
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
