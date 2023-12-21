use clap::Parser;
use std::time::{Duration, Instant};

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

    let base_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("Error creating client");

    for c in 0..args.count {
        println!("Call #{}", c);

        let address = args.address.clone();

        let client = base_client.clone();

        handles.spawn(async move {
            // let start = Instant::now();
            // thread::sleep(Duration::from_millis(1234));
            // let elapsed = start.elapsed();
            // println!(
            //     "start time: {:?}, duration {:?} ms\n",
            //     start,
            //     elapsed.as_millis()
            // );

            let start = Instant::now();
            let resp = client.get(address).send().await;

            match resp {
                Ok(success) => {
                    println!("{:#?}", success.status());
                    start.elapsed().as_millis()
                },
                Err(failure) => {
                    println!("Error requesting: {}, timeout: {}", failure, failure.is_timeout());

                    0
                },
            }
        });
    }

    while let Some(res) = handles.join_next().await {
        println!("Response time: {} ms", res.unwrap())
    }

    Ok(())

    // let resp = reqwest::get(&args.address).await?;
    // .text().await?;
    // println!("{:#?}, time: {:?} ms\n", resp.status(), elapsed.as_millis());
    // let worker = http_worker::HttpWorker {
    //     url: args.address,
    //     count: args.count,

    //     ..Default::default()
    // };

    // let res = worker.connect();

    // match res {
    //     Ok(response) => {
    //         println!("{}", response.response_time);
    //         ExitCode::SUCCESS
    //     }
    //     Err(error) => {
    //         println!("\nError executing worker!\n{}", error);
    //         ExitCode::FAILURE
    //     }
    // }
}
