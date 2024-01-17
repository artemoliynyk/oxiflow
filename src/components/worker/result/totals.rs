use crate::components::worker::http::response::HttpResponse;

/// Struct to count response number and average response time (ms) by code.
///
/// For example: store 2 responses and 120 ms as an average:
/// `ResponseCountAverage { count: 2, average: 120 };`
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

pub struct Totals {
    pub responses: ResponseCountAverage,
    pub errors: u32,
    pub skipped: u32,
    pub by_code: [ResponseCountAverage; 6],
}

impl Totals {
    /// calculate average reponse time from current average and success resonses
    pub fn count_response(&mut self, response: &HttpResponse) {
        self.responses.add_recalculate(response.response_time);

        self.count_particular_code(response);
    }

    pub fn inc_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn inc_error(&mut self) {
        self.errors += 1;
    }

    fn count_particular_code(&mut self, response: &HttpResponse) {
        if (100..599).contains(&response.code) {
            let http_group: usize = (response.code / 100) as usize;

            self.by_code[http_group].add_recalculate(response.response_time);
        } else {
            println!("Unexpected response code: {}", response.code);
        }
    }
}

impl Default for Totals {
    fn default() -> Self {
        let codes = [ResponseCountAverage::default(); 6];

        Self {
            responses: ResponseCountAverage::default(),
            errors: 0,
            skipped: 0,
            by_code: codes,
        }
    }
}
