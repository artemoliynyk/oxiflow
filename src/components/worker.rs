//! Worker – is a main "namespace" for all HTTP-related methods and implementation.
//!
//! This mod hosts HTTP client, response, result and error.
//!
//! See corresponding module docs for more details

pub mod request;
pub mod result;

use std::{thread, time::Duration, vec};

use crate::components::progressbar::Oxibar;
use log::{self};

use self::{request::WorkerRequest, result::WorkerResult};

use crate::components::http::{client::HttpClient, HttpResult};

pub struct Worker {
    http_client: &'static HttpClient,
    concurrent: u8,
    repeat: u8,
    delay: u8,
    queue_handles: tokio::task::JoinSet<HttpResult>,
}

impl Worker {
    pub fn new(http_client: &'static HttpClient, concurrent: u8, repeat: u8, delay: u8) -> Worker {
        let handles: tokio::task::JoinSet<HttpResult> = tokio::task::JoinSet::new();

        Worker {
            http_client,
            concurrent,
            repeat,
            delay,

            queue_handles: handles,
        }
    }

    /// Main method responsible for scheduling requests, waiting for them, recording the results and
    /// will show the progress or extra debug info.
    ///
    /// This method will check how many time to repeat, how many concurrent requests to perform,
    /// will perfor delay between repeats and will check the HTTP client reponse.
    ///
    /// All the responses will be checked and recorded in `WorkerResult` struct.
    pub async fn execute(&mut self, mut requests: Vec<WorkerRequest>) -> Box<WorkerResult> {
        if requests.is_empty() {
            panic!("No URLs found to call");
        }

        let concurrent = self.concurrent as usize;

        // fill-up the URL for the single address
        if requests.len() == 1 && self.concurrent > 1 {
            let url = requests.first().unwrap().clone();
            requests = vec![url; concurrent];
        }

        let req_len = requests.len();

        let mut result: Box<WorkerResult> = Box::new(WorkerResult::new());
        let mut progress_bar: Oxibar = Oxibar::new(requests.len() as u32 * self.repeat as u32);

        for iteration in 0..self.repeat {
            if self.repeat > 1 {
                log::info!(target: "worker::request", "Pass #{}", iteration + 1);
            }

            let step_size = match req_len <= concurrent {
                true => req_len,
                false => concurrent,
            };

            let mut start = 0;

            while start < req_len {
                let offset = start + step_size;
                let end = if offset < req_len { offset } else { req_len };

                let requests_slice = &requests[start..end];
                self.enqueue_requests(requests_slice, &mut result);
                self.join_queue(&mut result, &mut progress_bar).await;

                start = end;
            }

            if self.repeat > 0 && self.delay > 0 {
                log::info!("Waiting before repeating requests' batch {}s", self.delay);

                thread::sleep(Duration::from_secs(self.delay as u64));
            }
        }

        result
    }

    fn enqueue_requests(&mut self, requests: &[WorkerRequest], result: &mut Box<WorkerResult>) {
        for request in requests.iter() {
            self.http_client.resolve_request(request).map_or_else(
                |_| {
                    log::info!("Wrong HTTP method - skip and count skipped");

                    log::error!("Error calling URL - wrong method: '{}'", request.method);
                    result.totals.inc_skipped();
                },
                |req| {
                    let future = self.http_client.execute_request(req);
                    self.queue_handles.spawn(future);
                },
            );
        }
    }

    async fn join_queue(&mut self, result: &mut Box<WorkerResult>, progress_bar: &mut Oxibar) {
        while let Some(res) = self.queue_handles.join_next().await {
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
    }
}
