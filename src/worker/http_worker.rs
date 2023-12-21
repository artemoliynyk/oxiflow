use std::{future::Future, time::Duration};

use reqwest::{Client, ClientBuilder, RequestBuilder};

use super::WorkerResult;

/// HTTP specific worker, used to call HTTP/HTTPS urls
pub struct HttpWorker {
    pub repeat: u8,

    client: Client,
    request: Option<RequestBuilder>,
}

// impl Default for HttpWorker {
//     fn default() -> Self {
//         let default_timeout = Duration::from_secs(2);

//         Self {
//             url: "undefined".to_string(),
//             count: 1,

//             client: ,
//             request: None,
//         }
//     }
// }

impl Future for HttpWorker {
    type Output = WorkerResult;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl HttpWorker {
    fn new(repeat: u8, timeout_sec: u8) -> HttpWorker {
        let default_timeout = Duration::from_secs(timeout_sec as u64);

        HttpWorker {
            repeat,
            request: None,
            client: ClientBuilder::new()
                .timeout(default_timeout)
                .build()
                .expect("Error creating client"),
        }
    }
}

// pub trait HttpFlow {
//     // fn connect(&self) -> WorkerResult;
//     fn get(url: String, timeout: Duration) -> Self;
// }

// impl HttpFlow for HttpWorker {
// fn connect(&self) -> WorkerResult {
//     let error_msg = format!(
//         "Not implemented.\n\tURL: {}\n\tcount: {}\n\ttimeout: {}",
//         self.url, self.count, self.timeout
//     );

//     Err(WorkerError {
//         error: error_msg,
//         ..Default::default()
//     })
// }

// fn get(url: String, timeout: Duration) -> Self {

// }
// }
