//! Worker result module. This isn't `std::result::Result` and not related to it.
//!
//! This result stores how many actuall server reponses were receiver, how many failed
//! and what what the average response time for each HTTP code.

#![allow(clippy::print_stderr, clippy::print_stdout)]

use crate::components::http::{error::HttpError, response::HttpResponse};

use self::{single::Single, totals::Totals};

mod single;
mod totals;

#[derive(Default)]
pub struct WorkerResult {
    pub requests: Vec<Single>,
    pub totals: Totals,
}

impl WorkerResult {
    pub fn new() -> WorkerResult {
        WorkerResult::default()
    }

    pub fn success(&mut self, response: &HttpResponse) {
        self.totals.count_response(response);

        self.requests.push(Single::success(
            response.url.clone(),
            response.method.clone(),
            response.code,
            response.response_time,
        ));
    }

    pub fn failure(&mut self, response: &HttpError) {
        self.totals.inc_error();

        self.requests.push(Single::failure(
            response.url.clone(),
            response.method.clone(),
        ));
    }
}
