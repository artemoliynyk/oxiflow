#[derive(Default, Clone, Debug)]
pub struct WorkerRequest {
    pub method: String,
    pub url: String,
}

impl WorkerRequest {
    pub fn new(method: String, url: String) -> WorkerRequest {
        if !url.to_lowercase().starts_with("http") {
            panic!("Wrong URL - only HTTP-urls are supported: '{}'", url);
        }
        WorkerRequest { method, url }
    }
}
