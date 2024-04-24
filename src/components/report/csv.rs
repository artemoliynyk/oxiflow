use crate::components::worker::result::WorkerResult;

use super::Report;

pub struct ReportCsv {
    worker_result: &'static WorkerResult,
}

impl Report for ReportCsv {
    fn new<'a>(worker_result: &'static WorkerResult) -> Self {
        ReportCsv {
            worker_result
        }
    }

    fn set_filename(&self, base_name: String) -> String {
        todo!()
    }

    fn format_records(&self) -> Vec<String> {
        todo!()
    }
}