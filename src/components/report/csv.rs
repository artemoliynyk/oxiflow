use crate::components::worker::result::WorkerResult;

use super::Report;

pub struct ReportCsv {
    worker_result: &'static WorkerResult,
}

impl Report for ReportCsv {
    fn new<'a>(worker_result: &'static WorkerResult) -> Self {
        ReportCsv { worker_result }
    }

    fn set_filename(&mut self, base_name: String) {
        todo!()
    }

    fn get_filename(&mut self) -> String {
        todo!()
    }
    
    fn format_records(&self) -> Vec<String> {
        todo!()
    }
}
