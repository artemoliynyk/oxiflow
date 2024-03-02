mod tests {
    use oxiflow::components::{
        http::client::HttpClient,
        worker::{request::WorkerRequest, Worker},
    };

    fn setup() -> Worker {
        let http_client = Box::leak(Box::new(HttpClient::new(2)));
        Worker::new(http_client, 1, 1, 0)
    }

    #[tokio::test]
    #[should_panic]
    async fn test_execution() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> = Vec::new();

        worker.execute(requests).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_bad_urls() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> =
            vec![WorkerRequest::new("GET".into(), "not_an_address".into())];

        worker.execute(requests).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_bad_schema() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> =
            vec![WorkerRequest::new("GET".into(), "ftp://example.com".into())];

        worker.execute(requests).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_malformed_request() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> = vec![WorkerRequest::new(
            "BAD_METHOD".into(),
            "not_an_address".into(),
        )];

        let response = worker.execute(requests).await;
        assert!(response.totals.skipped == 2);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_empty_request() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> = vec![WorkerRequest::new(String::new(), String::new())];

        let response = worker.execute(requests).await;
        assert!(response.totals.skipped == 2);
    }

    #[tokio::test]
    async fn test_unreacheable_address() {
        let mut worker = setup();
        let requests: Vec<WorkerRequest> = vec![
            WorkerRequest::new("GET".into(), "http://no.sorry/".into()),
            WorkerRequest::new("TEST".into(), "http://no.sorry/".into()),
        ];

        let response = worker.execute(requests).await;
        assert!(response.totals.errors == 1);
        assert!(response.totals.skipped == 1);
    }
}
