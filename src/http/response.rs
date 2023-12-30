#[derive(Default)]
pub struct ClientResponse {
    /// HTTP response code
    pub code: u16,

    /// response time from the server in ms
    pub response_time: u128,
}
impl ClientResponse {
    /// Create new response
    pub fn new(res: reqwest::Response, elapsed: u128) -> ClientResponse {
        ClientResponse {
            code: res.status().as_u16(),
            response_time: elapsed,
        }
    }
}

impl std::fmt::Display for ClientResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Success: Status – {}, Response time - {} ms",
            self.code, self.response_time
        )
    }
}
