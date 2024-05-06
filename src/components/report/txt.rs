use std::io::{Error, Write};

use crate::components::worker::result::WorkerResult;

use super::Report;
pub struct ReportTxt {
    result: &'static WorkerResult,
    filename: String,
}

impl Report for ReportTxt {
    fn new(worker_result: &'static WorkerResult) -> Self {
        ReportTxt {
            result: worker_result,
            filename: String::new(),
        }
    }

    fn get_extenstion(&self) -> &str {
        "txt"
    }

    fn set_filename(&mut self, base_name: String) -> &String {
        self.filename = self.create_file(base_name);

        &self.filename
    }

    fn get_filename(&mut self) -> &String {
        &self.filename
    }

    fn write_report(&self) -> Result<u128, Error> {
        let mut file = self.open_file(&self.filename)?;

        for record in self.result.requests.iter() {
            let rep_status = match record.success {
                true => "HTTP".to_string(),
                false => record.timeout.map_or_else(
                    || "ERROR".to_string(),
                    |timeout_ms| format!("Timeout: {}ms", timeout_ms),
                ),
            };

            let rep_code = record
                .http_code
                .map_or_else(String::new, |code| format!("{}", code));

            let rep_time = record
                .time_ms
                .map_or_else(String::new, |time| format!("{}ms", time));

            let line = format!(
                "{method} {url}\n{status} {code} {time}\n\n",
                method = record.method,
                url = record.url,
                status = rep_status,
                code = rep_code,
                time = rep_time
            );
            let bytes_written = file.write(line.as_bytes())?;

            log::debug!("[REPORT][TXT]: Record: bytes written {}", bytes_written);
        }

        Ok(self.result.requests.len() as u128)
    }
}
