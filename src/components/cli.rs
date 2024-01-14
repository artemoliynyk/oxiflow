//! Helper class to build CLI args parser
use clap::Parser;
use env_logger::Builder as log_builder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Simple, fast, concurrent load tester with minimal reporting
pub struct Args {
    /// address to call
    pub address: String,

    /// which HTTP method to use for a call, try -mHELP to get list of supported methods
    #[arg(short, long, default_value_t = String::from("GET"))]
    pub method: String,

    /// how many request to send concurrently
    #[arg(short, long, default_value_t = 1)]
    pub concurrent: u8,

    /// how many times to repeat
    #[arg(short, long, default_value_t = 1)]
    pub repeat: u8,

    /// request timeout in seconds
    #[arg(short, long, default_value_t = 2)]
    pub timeout: u8,

    /// delay in seconds between repeating requests batches.
    /// Concurrent requests performed concurrently with no delay
    #[arg(short, long, default_value_t = 0)]
    pub delay: u8,

    /// print extra non-debug information
    #[arg(long)]
    pub verbose: bool,

    /// print debug information.
    /// if used with --verbose – enable a "trace mode", with a lot of extra info
    #[arg(long)]
    pub debug: bool,
}

pub struct Cli {
    pub args: Args,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            args: Args::parse(),
        }
    }
}
impl Cli {
    /// Create a Cli instance with all th eargs
    pub fn new() -> Cli {
        Cli::default()
    }

    // set log level based on the CLI args passed
    pub fn set_log_level(&self) {
        let log_level: log::LevelFilter = match (&self.args.verbose, &self.args.debug) {
            (true, true) => log::LevelFilter::Trace,
            (false, true) => log::LevelFilter::Debug,
            (true, false) => log::LevelFilter::Info,
            (false, false) => log::LevelFilter::Warn,
        };
        log_builder::new().filter_level(log_level).init();
        
        log::debug!("Error log level set to {}", log_level);
    }
}
