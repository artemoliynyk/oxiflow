#[derive(Default, Debug)]
pub struct WorkerRequest {
    pub method: String,
    pub address: String,
}

impl WorkerRequest {
    pub fn new(method: String, address: String) -> WorkerRequest {
        WorkerRequest { method, address }
    }
}
