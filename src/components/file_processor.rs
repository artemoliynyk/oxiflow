use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use super::worker::request::WorkerRequest;

pub struct FileProcessor<'a> {
    file_path: &'a str,
}

impl<'a> FileProcessor<'a> {
    pub fn new(file_path: &str) -> FileProcessor<'_> {
        FileProcessor { file_path }
    }

    pub fn read_urls(&self) -> Result<Vec<WorkerRequest>, Box<dyn Error>> {
        let lines = self.read_file()?;

        let mut requests: Vec<WorkerRequest> = Vec::new();

        for line in lines.flatten() {
            if line.is_empty() {
                continue;
            }

            if let Some(req) = self.parse_line(&line) {
                requests.push(req);
            }
        }

        if requests.is_empty() {
            return Err("File found, but not URLs recognised".into());
        }

        Ok(requests)
    }

    fn read_file(&self) -> std::result::Result<Lines<BufReader<File>>, Box<dyn std::error::Error>> {
        let path = Path::new(self.file_path.trim());

        if !path.exists() || !path.is_file() {
            return Err(format!("No file found: '{}'", self.file_path).into());
        }

        Ok(BufReader::new(File::open(path)?).lines())
    }

    fn parse_line(&self, line: &str) -> Option<WorkerRequest> {
        if let Some(pos) = line.trim().find('\u{20}') {
            let method = &line[0..pos];
            let url = &line[pos + 1..];

            return Some(WorkerRequest::new(method.to_string(), url.to_string()));
        }

        None
    }
}
