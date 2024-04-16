use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use super::worker::request::WorkerRequest;

#[derive(Default)]
pub struct FileProcessor<'a> {
    file_path: &'a str,
}

impl<'a> FileProcessor<'a> {
    pub fn new(file_path: &str) -> FileProcessor<'_> {
        FileProcessor { file_path }
    }

    #[allow(dead_code)]
    const fn mock() -> FileProcessor<'static> {
        FileProcessor { file_path: "" }
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
        if line.is_empty() {
            return None;
        }

        let mut method = "GET";
        let mut url = line.trim();

        // comment line
        if url.starts_with('#') {
            return None;
        }

        // any spaces may indicate method
        if let Some(pos) = url.find('\u{20}') {
            method = &url[0..pos];
            url = &url[pos + 1..];
        }
        Some(WorkerRequest::new(method.to_string(), url.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::FileProcessor;

    const MOCK_URL: &str = "http://example.net/test-123";
    const MOCK_PROCESSOR: FileProcessor<'_> = FileProcessor::mock();

    #[test]
    fn test_no_file() {
        let file_processor = FileProcessor::new("nonexistent.file");
        let res = file_processor.read_file();
        assert!(res.is_err());
    }

    #[test]
    fn test_line_parsing_correct_url() {
        let result = MOCK_PROCESSOR.parse_line("GET http://example.net/test-123");
        assert!(result.is_some());

        let req = result.unwrap();
        assert_eq!(req.clone().method, "GET");
        assert_eq!(req.clone().url, MOCK_URL);
    }

    #[test]
    fn test_line_parsing_wrong_method() {
        let result = MOCK_PROCESSOR.parse_line("OHNO http://example.net/test-123");
        assert!(result.is_some());

        let req = result.unwrap();
        assert_eq!(req.clone().method, "OHNO");
        assert_eq!(req.clone().url, MOCK_URL);
    }

    #[test]
    fn test_line_parsing_empty_line() {
        assert!(MOCK_PROCESSOR.parse_line("").is_none());
    }

    #[test]
    fn test_line_parsing_space_padded_url() {
        // with method
        let result = MOCK_PROCESSOR.parse_line(" http://example.net/test-123");
        assert!(result.is_some());

        let req = result.unwrap();
        assert_eq!(req.clone().method, "GET");
        assert_eq!(req.clone().url, MOCK_URL);
    }

    #[test]
    fn test_line_parsing_space_padded_method() {
        // no method
        let result = MOCK_PROCESSOR.parse_line(" POST http://example.net/test-123");
        assert!(result.is_some());

        let req = result.unwrap();
        assert_eq!(req.clone().method, "POST");
        assert_eq!(req.clone().url, MOCK_URL);
    }

    #[test]
    fn test_line_parsing_no_method() {
        let result = MOCK_PROCESSOR.parse_line("http://example.net/test-123");
        assert!(result.is_some());

        let req = result.unwrap();
        assert_eq!(req.clone().method, "GET");
        assert_eq!(req.clone().url, MOCK_URL);
    }
}
