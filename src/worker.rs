pub mod result;

use std::{thread, time::Duration};

use self::result::WorkerResult;
use crate::{http::client, progress::Oxibar};
use log;

pub const SUPPORTED_METHODS: [&str; 5] = ["GET", "POST", "DELETE", "PUT", "PATCH"];

pub async fn perform_requests(
    method: String,
    address: String,
    timeout: u8,
    concurrent: u8,
    repeat: u8,
    delay: u8,
) -> Box<WorkerResult> {
    let mut result = Box::new(WorkerResult::new());
    let mut handles: tokio::task::JoinSet<client::ClientResult> = tokio::task::JoinSet::new();

    let mut progress_bar = Oxibar::new(repeat as u32 * concurrent as u32);

    let worker = client::HttpClient::new(timeout);
    for iteration in 0..repeat {
        if repeat > 1 {
            log::info!(target: "worker::request", "Pass #{}", iteration + 1);
        }

        for _ in 0..concurrent {
            worker
                .resolve_request(method.clone(), address.clone())
                .map_or_else(
                    |_| {
                        log::info!("Wrong HTTP method - skip and count skipped");

                        log::error!("Error calling URL - wrong method: '{}'", method);
                        result.inc_skipped();
                    },
                    |req| {
                        let future = client::execute_request(req);
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
                    result.count_response(&client_response);
                    log::info!(target: "worker::request", "Response: {}", client_response);
                }
                Err(client_error) => {
                    result.inc_error();
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

pub fn is_supported_method(method: &str) -> bool {
    SUPPORTED_METHODS.contains(&method.trim().to_uppercase().as_str())
}
