use std::process::ExitCode;

use clap::Parser;
use oxiflow::worker::http::{self, HttpFlow};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// address to call
    address: String,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> ExitCode {
    let args = Args::parse();

    println!(
        "Calling target '{}' and repeat: {}",
        args.address, args.count
    );

    let worker = http::HttpWorker {
        url: args.address,
        count: args.count,

        ..Default::default()
    };

    let res = worker.connect();

    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            println!("\nError executing worker!\n{}", error);
            ExitCode::from(2)
        }
    }
}
