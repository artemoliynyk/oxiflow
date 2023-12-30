#[derive(Default)]
pub struct WorkerResult {
    pub average_response: u128,
    pub successes: u128,
    pub failures: u32,
}

impl WorkerResult {
    pub fn new() -> WorkerResult {
        WorkerResult::default()
    }

    pub fn count_success(&mut self, response_ms: u128) -> &Self {
        // new success calls count
        let successes = self.successes + 1;
        
        // calculate average reponse time from current average and success resonses
        self.average_response = (self.average_response * self.successes + response_ms) / successes;
        self.successes = successes;

        self
    }

    pub fn count_failure(&mut self) -> &Self {
        self.failures += 1;

        self
    }
}
