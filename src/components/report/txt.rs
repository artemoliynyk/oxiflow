use crate::components::worker::result::WorkerResult;

use super::Report;

pub struct ReportTxt {
    worker_result: &'static WorkerResult,
}

impl Report for ReportTxt {
    fn new(worker_result: &'static WorkerResult) -> Self {
        ReportTxt {
            worker_result: &worker_result
        }
    }

    fn set_filename(&self, base_name: String) -> String {
        todo!()
    }

    fn format_records(&self) -> Vec<String> {
        todo!()
    }
    
}