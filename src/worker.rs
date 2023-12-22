use std::time::Instant;

use reqwest::RequestBuilder;

pub mod http_worker;

#[derive(Default)]
pub struct WorkerResponse {
    /// HTTP response code
    pub code: u16,

    /// response time from the server in ms
    pub response_time: u128,
}

#[derive(Debug, Default)]
pub struct WorkerError {
    error: String,
    is_timeout: bool,

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u128>,
}

impl std::fmt::Display for WorkerError {
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

impl std::fmt::Display for WorkerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Seccess: Status – {}, Response time (ms) - {}",
            self.code, self.response_time
        )
    }
}

pub type WorkerResult = std::result::Result<WorkerResponse, WorkerError>;

pub async fn execute_request(request: RequestBuilder) -> WorkerResult {
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
