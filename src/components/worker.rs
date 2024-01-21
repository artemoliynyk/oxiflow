//! Worker – is a main "namespace" for all HTTP-related methods and implementation.
//!
//! This mod hosts HTTP client, response, result and error.
//!
//! See corresponding module docs for more details

pub mod result;

use std::{thread, time::Duration};

use crate::components::progressbar::Oxibar;
use log;

use self::result::WorkerResult;

use super::http::{HttpResult, client::HttpClient};

/// Main method responsible for scheduling requests, waiting for them, recording the results and
/// will show the progress or extra debug info.
///
/// This method will check how many time to repeat, how many concurrent requests to perform,
/// will perfor delay between repeats and will check the HTTP client reponse.
///
/// All the responses will be checked and recorded in `WorkerResult` struct.
pub async fn perform_requests(
    method: String,
    address: String,
    timeout: u8,
    concurrent: u8,
    repeat: u8,
    delay: u8,
) -> Box<WorkerResult> {
    let mut result = Box::new(WorkerResult::new());
    let mut handles: tokio::task::JoinSet<HttpResult> = tokio::task::JoinSet::new();

    let mut progress_bar = Oxibar::new(repeat as u32 * concurrent as u32);

    let http_client = HttpClient::new(timeout);

    for iteration in 0..repeat {
        if repeat > 1 {
            log::info!(target: "worker::request", "Pass #{}", iteration + 1);
        }

        for _ in 0..concurrent {
            http_client
                .resolve_request(method.clone(), address.clone())
                .map_or_else(
                    |_| {
                        log::info!("Wrong HTTP method - skip and count skipped");

                        log::error!("Error calling URL - wrong method: '{}'", method);
                        result.totals.inc_skipped();
                    },
                    |req| {
                        let future = super::http::client::execute_request(req);
                        handles.spawn(future);
                    },
                );
        }

        while let Some(res) = handles.join_next().await {
            if log::max_level() <= log::Level::Warn {
                progress_bar.advance().print();
            }

            match res.unwrap() {
                Ok(client_response) => {
                    result.success(&client_response);
                    log::info!(target: "worker::request", "Response: {}", client_response);
                }
                Err(client_error) => {
                    result.failure(&client_error);
                    log::info!(target: "worker::request", "Failed: {}", client_error);
                }
            }
        }

        if repeat > 0 && delay > 0 {
            log::info!("Waiting before repeating requests' batch {}s", delay);

            thread::sleep(Duration::from_secs(delay as u64));
        }
    }

    result
}
