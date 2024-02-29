//! Main namespace module to keep HTTP related sctructs: Client and its Reponse and Error
pub mod client;
pub mod error;
pub mod response;

use self::{error::HttpError, response::HttpResponse};

pub type HttpResult = std::result::Result<HttpResponse, HttpError>;

/// supported HTTP-methods, used for the command line args filtering
pub const SUPPORTED_HTTP_METHODS: [&str; 5] = ["GET", "POST", "DELETE", "PUT", "PATCH"];

/// check if method arg passed from the command line is valid and supported
pub fn method_supported(method: &str) -> bool {
    SUPPORTED_HTTP_METHODS.contains(&method.trim().to_uppercase().as_str())
}

/// check if method arg passed from the command line is valid and supported
pub fn list_methods() -> String {
    SUPPORTED_HTTP_METHODS.join(", ")
}