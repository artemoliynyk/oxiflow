//! Report components. Printing, exporting and summarising session results
#![allow(clippy::print_stderr, clippy::print_stdout)]

use self::{csv::ReportCsv, txt::ReportTxt};

use super::{cli::ReportFormats, worker::result::WorkerResult};

pub mod csv;
pub mod output;
pub mod txt;

trait Report {
    fn new(worker_result: &'static WorkerResult) -> Self
    where
        Self: Sized;
    fn set_filename(&self, base_name: String) -> String;
    fn format_records(&self) -> Vec<String>;
}

pub fn create_report(format: &ReportFormats, worker_result: &'static WorkerResult) {
    let report: Box<dyn Report> = match &format {
        ReportFormats::Csv => Box::new(ReportCsv::new(worker_result)),
        ReportFormats::Txt => Box::new(ReportTxt::new(worker_result)),
    };

    report.set_filename("base_name".to_string());
}
