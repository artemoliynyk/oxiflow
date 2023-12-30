pub mod result;

use crate::http::client;

use self::result::WorkerResult;

pub async fn perform_requests(
    address: String,
    timeout: u8,
    concurrent: u8,
    repeat: u8,
) -> Box<WorkerResult> {
    let mut result = Box::new(WorkerResult::new());

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
                Ok(client_response) => {
                    result.count_response(&client_response);
                    println!("Response: {}", client_response);
                }
                Err(client_error) => {
                    result.count_error();
                    println!("Failed: {}", client_error)
                }
            }
        }
        println!(" ");
    }

    result
}
