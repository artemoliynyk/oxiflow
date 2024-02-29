#![allow(clippy::print_stderr, clippy::print_stdout)]

use std::process::ExitCode;

use oxiflow::components::cli::Cli;
use oxiflow::components::http::client::HttpClient;
use oxiflow::components::report;
use oxiflow::components::worker::request::WorkerRequest;
use oxiflow::components::worker::result::WorkerResult;
use oxiflow::components::worker::Worker;

fn main() -> ExitCode {
    let cli_tools = Cli::create();
    if let Err(exit_code) = cli_tools {
        return ExitCode::from(exit_code);
    }
    let cli = cli_tools.unwrap();

    println!("Calling target: {} {}", &cli.args.method, &cli.args.address);
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec",
        &cli.args.concurrent, &cli.args.repeat, &cli.args.timeout, &cli.args.delay
    );
    println!();

    let http_client = Box::leak(Box::new(HttpClient::new(cli.args.timeout)));
    let worker = Worker::new(
        http_client,
        cli.args.concurrent,
        cli.args.repeat,
        cli.args.delay,
    );

    // aync runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result: Box<WorkerResult> = rt.block_on(async {
        let request = WorkerRequest::new(cli.args.method, cli.args.address);
        worker
            .perform_requests(request)
            .await
    });
    println!();

    let report = report::Report::new(&result);

    report.print_report();

    ExitCode::SUCCESS
}
