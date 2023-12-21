use clap::Parser;
use oxiflow::worker::http_worker::HttpWorker;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// address to call
    address: String,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        "Calling target '{}' and repeat: {}",
        &args.address, &args.count
    );

    let mut handles: tokio::task::JoinSet<u128> = tokio::task::JoinSet::new();

    for c in 0..args.count {
        println!("Call #{}", c);

        let address = args.address.clone();

        handles.spawn(async move {
            let start = Instant::now();

            let worker = HttpWorker::new(5);

            let resp = worker.get(address).await;

            match resp {
                Ok(success) => {
                    println!("{:#?}", success.status());
                    start.elapsed().as_millis()
                }
                Err(failure) => {
                    println!(
                        "Error requesting: {}, timeout: {}",
                        failure,
                        failure.is_timeout()
                    );

                    0
                }
            }
        });
    }

    while let Some(res) = handles.join_next().await {
        println!("Response time: {} ms", res.unwrap())
    }

    Ok(())
}
