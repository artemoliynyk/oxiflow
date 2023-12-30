use crate::http::response::ClientResponse;

/// Struct to count response number and average response time (ms) by code.
///
/// ```
/// // store 2 responses and 120 ms as an average
/// ResponseCountAverage {
///     count: 2,
///     average: 120,
/// };
/// ```
#[derive(Default, Clone, Copy)]
pub struct ResponseCountAverage {
    pub count: u32,
    pub average_ms: u128,
}

impl ResponseCountAverage {
    /// Increase response count and re-calculate average
    pub fn add_recalculate(&mut self, time_ms: u128) {
        let new_count = self.count + 1;

        let new_avg = (self.average_ms * self.count as u128 + time_ms) / new_count as u128;

        self.count = new_count;
        self.average_ms = new_avg;
    }
}

pub struct WorkerResult {
    // pub average_response: u128,
    pub total_responces: ResponseCountAverage,
    pub total_errors: u32,
    pub total_by_code: [ResponseCountAverage; 6],
}

impl Default for WorkerResult {
    fn default() -> Self {
        let codes = [ResponseCountAverage::default(); 6];

        WorkerResult {
            total_responces: ResponseCountAverage::default(),
            total_errors: 0,
            total_by_code: codes,
        }
    }
}

impl WorkerResult {
    pub fn new() -> WorkerResult {
        WorkerResult::default()
    }

    /// calculate average reponse time from current average and success resonses
    pub fn count_response(&mut self, response: &ClientResponse) {
        self.total_responces.add_recalculate(response.response_time);

        self.count_particular_code(response);
    }

    pub fn count_error(&mut self) {
        self.total_errors += 1;
    }

    fn count_particular_code(&mut self, response: &ClientResponse) {
        if (100..599).contains(&response.code) {
            let http_group: usize = (response.code / 100) as usize;

            self.total_by_code[http_group].add_recalculate(response.response_time);
        } else {
            println!("Unexpected response code: {}", response.code);
        }
    }
}
