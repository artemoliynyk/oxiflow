use clap::Parser;
use env_logger::Builder as log_builder;
use oxiflow::worker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Simple, fast, concurrent load tester with minimal reporting
struct Args {
    /// address to call
    address: String,

    /// how many request to send concurrently
    #[arg(short, long, default_value_t = 1)]
    concurrent: u8,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    repeat: u8,

    /// request timeout in seconds
    #[arg(short, long, default_value_t = 2)]
    timeout: u8,

    /// print extra non-debug information
    #[arg(short, long)]
    verbose: bool,

    /// print debug information.
    /// if used with --verbose – enable a "trace mode", with a lot of extra info
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let log_level: log::LevelFilter = match (args.verbose, args.debug) {
        (true, true) => log::LevelFilter::Trace,
        (false, true) => log::LevelFilter::Debug,
        (true, false) => log::LevelFilter::Info,
        (false, false) => log::LevelFilter::Warn,
    };
    log_builder::new().filter_level(log_level).init();

    println!(
        "Calling target '{}', concurren clients: {}, repeat: {}, timeout {} sec.",
        &args.address, &args.concurrent, &args.repeat, &args.timeout
    );

    let result =
        worker::perform_requests(args.address, args.timeout, args.concurrent, args.repeat).await;

    println!("{} Results {}", "=".repeat(13), "=".repeat(13));
    println!("Successes: {}", result.total_responces.count);
    println!("Failures: {}", result.total_errors);
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
