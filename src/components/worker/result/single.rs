//! This stcut represents result for a single request

// TODO: add request start time

#[derive(Default, serde::Serialize)]
pub struct Single {
    pub success: bool,
    pub url: String,
    pub method: String,
    pub http_code: Option<u16>,
    pub time_ms: Option<u128>,
    pub timeout: Option<u128>,
}

impl Single {
    pub fn success(url: String, method: String, response_code: u16, elapsed: u128) -> Single {
        Single {
            success: true,
            url,
            method,
            http_code: Some(response_code),
            time_ms: Some(elapsed),
            timeout: None,
        }
    }

    pub fn failure(url: String, method: String, timeout: Option<u128>) -> Single {
        Single {
            success: false,
            url,
            method,
            timeout,
            ..Default::default()
        }
    }
}
