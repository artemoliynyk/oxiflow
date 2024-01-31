#![allow(clippy::print_stderr, clippy::print_stdout)]

use oxiflow::components::cli::Cli;
use oxiflow::components::http;
use oxiflow::components::http::client::HttpClient;
use oxiflow::components::report;
use oxiflow::components::worker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_tools = get_cli_tools();
    if let Err(err) = cli_tools {
        return Err(err.into());
    }
    let cli = cli_tools.unwrap();

    println!("Calling target: {} {}", &cli.args.method, &cli.args.address);
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec",
        &cli.args.concurrent, &cli.args.repeat, &cli.args.timeout, &cli.args.delay
    );
    println!();

    let http_client = Box::leak(Box::new(HttpClient::new(cli.args.timeout)));

    // aync runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result: Box<worker::result::WorkerResult> = rt.block_on(async {
        worker::perform_requests(
            http_client,
            cli.args.method,
            cli.args.address,
            cli.args.concurrent,
            cli.args.repeat,
            cli.args.delay,
        )
        .await
    });
    println!();

    let report = report::Report::new(&result);

    report.print_report();
    Ok(())
}

fn get_cli_tools() -> Result<Cli, String> {
    let cli = match Cli::from_os_env() {
        Ok(instance) => instance,
        Err(err) => {
            err.print().expect("Unable to format error details");
            return Err("Wrong arguments".to_string());
        }
    };
    cli.set_log_level();

    if !http::client::is_supported_method(&cli.args.method) {
        println!(
            "Supported methods: {}",
            http::client::SUPPORTED_METHODS.join(", ")
        );

        return Err(format!(
            "Defined method is not supported '{}'",
            &cli.args.method
        ));
    }

    if cli.args.repeat > 0 && cli.args.delay >= 30 {
        println!(
            "Warning: delay is set to {}s, it seems unreasonably high\n",
            &cli.args.delay
        );
    }

    if !cli.args.file.is_empty() {
        return Err(format!("File option is not supported yet, ignoring '{}'", cli.args.file));
    }

    Ok(cli)
}
