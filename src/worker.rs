pub mod http_worker;

pub enum WorkerType {
    Http,
}

#[derive(Default)]
pub struct WorkerResponse {
    /// HTTP response code
    pub code: u16,

    /// response time from the server in ms
    pub response_time: u32,
}

#[derive(Debug, Default)]
pub struct WorkerError {
    error: String,

    /// if error happened due to the timeout – this field will hold the time (ms)
    timeout: Option<u32>,
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timeout(ms): {}\nError: {}",
            self.timeout.unwrap_or_default(), self.error
        )
    }
}

type WorkerResult = std::result::Result<WorkerResponse, WorkerError>;
