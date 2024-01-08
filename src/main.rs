use clap::Parser;
use env_logger::Builder as log_builder;
use oxiflow::components::worker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Simple, fast, concurrent load tester with minimal reporting
struct Args {
    /// address to call
    address: String,

    /// which HTTP method to use for a call, try -mHELP to get list of supported methods
    #[arg(short, long, default_value_t = String::from("GET"))]
    method: String,

    /// how many request to send concurrently
    #[arg(short, long, default_value_t = 1)]
    concurrent: u8,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    repeat: u8,

    /// request timeout in seconds
    #[arg(short, long, default_value_t = 2)]
    timeout: u8,

    /// delay in seconds between repeating requests batches.
    /// Concurrent requests performed concurrently with no delay
    #[arg(short, long, default_value_t = 0)]
    delay: u8,

    /// print extra non-debug information
    #[arg(long)]
    verbose: bool,

    /// print debug information.
    /// if used with --verbose – enable a "trace mode", with a lot of extra info
    #[arg(long)]
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

    if !worker::is_supported_method(&args.method) {
        println!("Defined method is not supported '{}'", &args.method);
        println!(
            "Supported methods: {}",
            worker::SUPPORTED_METHODS.join(", ")
        );

        return Err("Wrong argument".into());
    }

    println!("Calling target: {} {}", &args.method, &args.address);
    println!(
        "Concurren clients: {}\nRepeat: {}\nTimeout: {} sec\nDelay: {} sec",
        &args.concurrent, &args.repeat, &args.timeout, &args.delay
    );

    if args.repeat > 0 && args.delay >= 30 {
        println!(
            "\n{}\nWarning: you have set delay between repeats to {}s",
            "=".repeat(45),
            &args.delay,
        );
        println!(
            "It seems like unreasonable high delay\n{}\n",
            "=".repeat(45)
        );
    }

    let result = worker::perform_requests(
        args.method,
        args.address,
        args.timeout,
        args.concurrent,
        args.repeat,
        args.delay,
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
