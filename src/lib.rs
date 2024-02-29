pub mod app_info {
    pub const NAME: &str = "Oxiflow";
    pub const VERSION: &str = "0.1.5";
    pub const AUTHOR: &str = "Artem Oliinyk <azure.email@gmail.com";
    pub const DESCRIPTION: &str = "Small fast HTTP load-testing app";
}
pub mod components;

/// supported HTTP-methods, used for the command line args filtering
pub const SUPPORTED_HTTP_METHODS: [&str; 5] = ["GET", "POST", "DELETE", "PUT", "PATCH"];

/// check if method arg passed from the command line is valid and supported
pub fn method_supported(method: &str) -> bool {
    SUPPORTED_HTTP_METHODS.contains(&method.trim().to_uppercase().as_str())
}