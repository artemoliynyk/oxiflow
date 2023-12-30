use clap::Parser;
use oxiflow::worker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Simple, fast, concurrent load tester with minimal reporting
struct Args {
    /// address to call
    address: String,

    /// how many request to send concurrently
    #[arg(short, long, default_value_t = 1)]
    concurrent: u16,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    repeat: u16,

    /// request timeout in seconds
    #[arg(short, long, default_value_t = 2)]
    timeout: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        "Calling target '{}', concurren clients: {}, repeat: {}, timeout {} sec.",
        &args.address, &args.concurrent, &args.repeat, &args.timeout
    );

    let result =
        worker::perform_requests(args.address, args.timeout, args.concurrent, args.repeat).await;

    println!("Successes: {}", result.successes);
    println!("Failures: {}", result.failures);
    println!("Average response time (ms): {}", result.average_response);

    Ok(())
}
