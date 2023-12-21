use clap::Parser;
use oxiflow::worker::{http_worker::HttpWorker, WorkerResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// address to call
    address: String,

    /// how many request to send concurrently
    #[arg(short, long, default_value_t = 1)]
    concurrent: u16,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    repeat: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        "Calling target '{}' and repeat: {}",
        &args.address, &args.concurrent
    );

    let mut handles: tokio::task::JoinSet<WorkerResult> = tokio::task::JoinSet::new();

    for iteration in 0..args.repeat {
        if args.repeat > 1 {
            println!("Pass #{}", iteration + 1);
        }

        for _ in 0..args.concurrent {
            let worker = HttpWorker::new(2);
            let req = worker.get(args.address.clone());

            let future = worker.execute(req);
            handles.spawn(future);
        }

        while let Some(res) = handles.join_next().await {
            match res.unwrap() {
                Ok(ok) => {
                    println!("Response: {}", ok)
                }
                Err(err) => println!("Failed: {}", err),
            }
        }
    }

    Ok(())
}
