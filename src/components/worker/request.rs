#[derive(Default, Clone, Debug)]
pub struct WorkerRequest {
    pub method: String,
    pub url: String,
}

impl WorkerRequest {
    pub fn new(method: String, url: String) -> WorkerRequest {
        WorkerRequest { method, url }
    }
}
