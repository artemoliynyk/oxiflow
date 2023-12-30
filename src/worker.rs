pub mod result;

use crate::http::client;

use self::result::WorkerResult;

pub async fn perform_requests(
    address: String,
    timeout: u8,
    concurrent: u16,
    repeat: u16,
) -> WorkerResult {
    let mut result = WorkerResult::new();

    let mut handles: tokio::task::JoinSet<client::ClientResult> = tokio::task::JoinSet::new();

    let worker = client::HttpClient::new(timeout);
    for iteration in 0..repeat {
        if repeat > 1 {
            println!("Pass #{}", iteration + 1);
        }

        for _ in 0..concurrent {
            let req = worker.get(address.clone());
            let future = client::execute_request(req);
            handles.spawn(future);
        }

        while let Some(res) = handles.join_next().await {
            match res.unwrap() {
                Ok(ok) => {
                    result.count_success(ok.response_time);
                    println!("Response: {}", ok);
                }
                Err(err) => {
                    result.count_failure();
                    println!("Failed: {}", err)
                }
            }
        }
        println!(" ");
    }

    result
}
