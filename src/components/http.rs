//! Main namespace module to keep HTTP related sctructs: Client and its Reponse and Error
pub mod client;
pub mod error;
pub mod response;

use self::{error::HttpError, response::HttpResponse};

pub type HttpResult = std::result::Result<HttpResponse, HttpError>;
