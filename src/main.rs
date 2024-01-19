#![allow(clippy::print_stderr, clippy::print_stdout)]

use oxiflow::components::cli::Cli;
use oxiflow::components::report;
use oxiflow::components::worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = match Cli::from_os_env() {
        Ok(instance) => instance,
        Err(err) => {
            err.print().expect("Unable to format error details");
            return Err("Wrong arguments".into());
        }
    };
    cli.set_log_level();

    if !worker::is_supported_method(&cli.args.method) {
        println!(
            "Supported methods: {}",
            worker::SUPPORTED_METHODS.join(", ")
        );

        return Err(format!("Defined method is not supported '{}'", &cli.args.method).into());
    }

    if cli.args.repeat > 0 && cli.args.delay >= 30 {
        println!(
            "Warning: delay is set to {}s, it seems unreasonably high\n",
            &cli.args.delay
        );
    }

    println!("Calling target: {} {}", &cli.args.method, &cli.args.address);
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec",
        &cli.args.concurrent, &cli.args.repeat, &cli.args.timeout, &cli.args.delay
    );
    println!();

    let result = worker::perform_requests(
        cli.args.method,
        cli.args.address,
        cli.args.timeout,
        cli.args.concurrent,
        cli.args.repeat,
        cli.args.delay,
    )
    .await;

    println!();

    let report = report::Report::new(&result);

    report.print_report();

    Ok(())
}
