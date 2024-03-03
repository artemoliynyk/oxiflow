#![allow(clippy::print_stderr, clippy::print_stdout)]

use std::error::Error;
use std::process::ExitCode;

use oxiflow::components::cli::{Args, Cli};
use oxiflow::components::file_processor::FileProcessor;
use oxiflow::components::http::client::HttpClient;
use oxiflow::components::report;
use oxiflow::components::worker::request::WorkerRequest;
use oxiflow::components::worker::result::WorkerResult;
use oxiflow::components::worker::Worker;
use oxiflow::EXIT_NO_URLS_FOUND;

fn main() -> ExitCode {
    let cli_tools = Cli::create();
    if let Err(exit_code) = cli_tools {
        return ExitCode::from(exit_code);
    }
    let args = cli_tools.unwrap().args;

    let requests = parse_requests(&args);
    if let Err(error) = requests {
        println!("No URLs to call");
        println!("Error: {}", error);
        return ExitCode::from(EXIT_NO_URLS_FOUND);
    }

    print_intro(&args);

    let http_client = Box::leak(Box::new(HttpClient::new(args.timeout)));
    let mut worker = Worker::new(http_client, args.concurrent, args.repeat, args.delay);

    // async runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result: Box<WorkerResult> = rt.block_on(async { worker.execute(requests.unwrap()).await });
    println!();

    let report = report::Report::new(&result);
    report.print_report();

    ExitCode::SUCCESS
}

fn parse_requests(args: &Args) -> Result<Vec<WorkerRequest>, Box<dyn Error>> {
    if !args.url.is_empty() {
        Ok(vec![WorkerRequest::new(
            args.method.clone(),
            args.url.clone(),
        )])
    } else {
        FileProcessor::new(&args.file).read_urls()
    }
}
fn print_intro(args: &Args) {
    if args.file.is_empty() {
        println!("Calling target: {} {}", &args.method, &args.url);
    }
    
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec\n",
        &args.concurrent, &args.repeat, &args.timeout, &args.delay
    );
}
