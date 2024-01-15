#![allow(clippy::print_stderr, clippy::print_stdout)]

use oxiflow::components::cli::Cli;
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
        println!("Defined method is not supported '{}'", &cli.args.method);
        println!(
            "Supported methods: {}",
            worker::SUPPORTED_METHODS.join(", ")
        );

        return Err("Wrong argument".into());
    }

    println!("Calling target: {} {}", &cli.args.method, &cli.args.address);
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec",
        &cli.args.concurrent, &cli.args.repeat, &cli.args.timeout, &cli.args.delay
    );

    if cli.args.repeat > 0 && cli.args.delay >= 30 {
        println!(
            "\n{}\nWarning: you have set delay between repeats to {}s",
            "=".repeat(45),
            &cli.args.delay,
        );
        println!(
            "It seems like unreasonable high delay\n{}\n",
            "=".repeat(45)
        );
    }

    let result = worker::perform_requests(
        cli.args.method,
        cli.args.address,
        cli.args.timeout,
        cli.args.concurrent,
        cli.args.repeat,
        cli.args.delay,
    )
    .await;

    println!("{} Results {}", "=".repeat(13), "=".repeat(13));
    println!("Successes: {}", result.total_responces.count);
    println!("Failures: {}", result.total_errors);
    println!("Skipped: {}", result.total_skipped);
    println!(
        "Average response time: {} ms",
        result.total_responces.average_ms
    );
    println!(" ");
    println!("{} Stats by code {}", "=".repeat(10), "=".repeat(10));
    println!("Code\t\tResponses\tAverage time (ms)");

    for i in 1u8..6u8 {
        let code_data = result.total_by_code.get(i as usize).unwrap();
        println!(
            "HTTP {}xx\t{}\t\t{}",
            i, code_data.count, code_data.average_ms
        );
    }
    println!("{}", "=".repeat(35));

    Ok(())
}
