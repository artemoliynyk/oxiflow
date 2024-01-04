pub mod result;

use self::result::WorkerResult;
use crate::{http::client, progress::Oxibar};
use log;

pub async fn perform_requests(
    address: String,
    timeout: u8,
    concurrent: u8,
    repeat: u8,
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
            let req = worker.get(address.clone());
            let future = client::execute_request(req);
            handles.spawn(future);
        }

        while let Some(res) = handles.join_next().await {
            progress_bar.advance().print();

            match res.unwrap() {
                Ok(client_response) => {
                    result.count_response(&client_response);
                    log::info!(target: "worker::request", "Response: {}", client_response);
                }
                Err(client_error) => {
                    result.count_error();
                    log::info!(target: "worker::request", "Failed: {}", client_error);
                }
            }
        }
    }

    result
}
