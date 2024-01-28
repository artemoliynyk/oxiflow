//! Report component. Printing, exporting and summarising session results
#![allow(clippy::print_stderr, clippy::print_stdout)]

use super::worker::result::WorkerResult;
pub struct Report<'a> {
    worker_result: &'a WorkerResult,
}

impl<'a> Report<'a> {
    const REPORT_WIDTH: usize = 61;

    pub fn new(result: &WorkerResult) -> Report<'_> {
        Report {
            worker_result: result,
        }
    }
    fn get_filler(&self, title: &str) -> String {
        let filler_size = (Self::REPORT_WIDTH - title.len() - 2) / 2;

        "=".repeat(filler_size)
    }

    fn print_into(&self, title: &str) {
        let filler_part = self.get_filler(title);

        println!("{} {} {}", filler_part, title, filler_part);
    }

    fn print_end(&self) {
        println!("{}", "=".repeat(Self::REPORT_WIDTH));
    }

    fn print_summary(&self) {
        println!("Successes: {}", self.worker_result.totals.responses.count);
        println!("Failures: {}", self.worker_result.totals.errors);
        println!("Skipped: {}", self.worker_result.totals.skipped);
        println!(
            "Average response time: {} ms",
            self.worker_result.totals.responses.average_ms
        );
        println!();
    }

    fn print_per_code_result(&self) {
        self.print_into("Stats by code");

        println!("Code\t\tResponses\tAverage time (ms)");

        for i in 1u8..6u8 {
            let code_data = self.worker_result.totals.by_code.get(i as usize).unwrap();
            println!(
                "HTTP {}xx\t{}\t\t{}",
                i, code_data.count, code_data.average_ms
            );
        }
    }

    pub fn print_report(&self) {
        self.print_into("Results");
        self.print_summary();
        self.print_per_code_result();

        println!();
        self.print_per_request();

        self.print_end();
    }

    pub fn print_per_request(&self) {
        self.print_into("Per-request results");

        for record in self.worker_result.requests.iter() {
            let (code, time) = match record.success {
                true => {
                    let code = record.http_code.unwrap();

                    (
                        reqwest::StatusCode::from_u16(code).unwrap().to_string(),
                        record.time_ms.unwrap_or_default().to_string(),
                    )
                }
                false => ("Failed".to_string(), "0".to_string()),
            };
            println!(
                "{: >7} ms {: >7} {} - {}",
                time, record.method, record.url, code
            );
        }
    }
}
