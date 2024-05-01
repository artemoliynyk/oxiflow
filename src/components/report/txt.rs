use std::{
    fs::File,
    io::{Error, Write},
};

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
            let line = format!("{} {}ms {} {}\n", record.success, record.time_ms.unwrap_or(0), record.method, record.url);
            let bytes_written = file.write(line.as_bytes())?;

            log::debug!("REPORT: Record: bytes written {}", bytes_written);
        }

        return Ok(self.result.requests.len() as u128);
    }
}
