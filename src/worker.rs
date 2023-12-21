pub mod http_worker;

pub enum WorkerType {
    Http,
}

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

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u128>,
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Timeout(ms) - {}",
            self.error,
            self.timeout.unwrap_or_default(),
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
