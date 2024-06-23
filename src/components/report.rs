//! Report components. Printing, exporting and summarising session results
#![allow(clippy::print_stderr, clippy::print_stdout)]
use std::{fs::File, io::Error, path::Path};

use chrono::Local;

use self::txt::ReportTxt;

use super::{cli::ReportFormats, worker::result::WorkerResult};

// pub mod csv;
pub mod output;
pub mod txt;

trait Report {
    fn new(worker_result: &'static WorkerResult) -> Self
    where
        Self: Sized;

    /// Method to return report-related file extenstion (without leading dot, for e.g.: `txt`)
    fn get_extenstion(&self) -> &str;

    /// Set base filename for the report – this method must resolve full unique file name with
    /// extenstion, for example: `report-20240501-1.txt`
    fn set_filename(&mut self, base_name: String) -> &String;

    /// Return resolved unique full file name set by `Report::set_filename()`
    fn get_filename(&mut self) -> &String;

    /// This method to write report, it must return success with number of rows/records written
    /// or error if any occured
    fn write_report(&self) -> Result<u128, Error>;

    /// Helper method: resolves unique full file name with extenstion for current report
    fn create_file(&self, base_name: String) -> String {
        let mut base: String = format!("{}.{}", base_name, self.get_extenstion());

        let mut counter = 0;
        while Path::new(base.as_str()).exists() {
            counter += 1;
            base = format!("{}-{}.{}", base_name, counter, self.get_extenstion());
        }

        base
    }

    fn open_file(&self, filename: &str) -> Result<File, std::io::Error> {
        let f = File::create(filename);

        match f {
            Ok(file) => Ok(file),
            Err(error) => {
                log::warn!(
                    "Error opening file '{filename}' for writing report: {}",
                    error
                );

                Err(error)
            }
        }
    }
}

fn format_current_time() -> String {
    Local::now().format("%Y%m%d-%H%M%S").to_string()
}

fn get_base_filename() -> String {
    format!("oxiflow_report_{}", format_current_time())
}

pub fn create_report(format: &ReportFormats, worker_result: &'static WorkerResult) {
    let mut report: Box<dyn Report> = match &format {
        ReportFormats::Txt => Box::new(ReportTxt::new(worker_result)),
        ReportFormats::Csv => todo!(),
    };

    let base_name = get_base_filename();
    report.set_filename(base_name);

    match report.write_report() {
        Ok(report_lines) => println!(
            "Report file saved '{}', {} line(s) was written",
            report.get_filename(),
            report_lines
        ),
        Err(_) => println!("Unable to save report in file '{}'", report.get_filename()),
    }
}
