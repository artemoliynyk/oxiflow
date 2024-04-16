//! Helper class to build CLI args parser

#![allow(clippy::print_stderr, clippy::print_stdout)]
use std::{ffi::OsString, vec::IntoIter};

use clap::{ArgAction, Parser};
use env_logger::Builder as log_builder;

use crate::components::http;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Simple, fast, concurrent load tester with minimal reporting
pub struct Args {
    /// singe URL to call
    #[arg(
        conflicts_with("file"),
        required_unless_present("file"),
        required_unless_present("help_methods"),
        required_unless_present("help_file"),
        default_value = ""
    )]
    pub url: String,

    /// HTTP method to use for calling the singe URL
    #[arg(long, short('m'), conflicts_with("file"), default_value = "GET")]
    pub method: String,

    /// text file with methods (optional) and URLs call, lines format: [METHOD] <URL>
    #[arg(
        long,
        short('f'),
        conflicts_with("url"),
        required_unless_present("url"),
        required_unless_present("help_methods"),
        required_unless_present("help_file"),
        default_value = ""
    )]
    pub file: String,

    /// how many request to send concurrently
    #[arg(long, short('c'), default_value_t = 1)]
    pub concurrent: u8,

    /// how many times to repeat either single URL call or of all URLs in file.
    #[arg(long, short('r'), default_value_t = 1)]
    pub repeat: u8,

    /// (seconds) request timeout, all requests lasting longer will be cancelled
    #[arg(long, short('t'), default_value_t = 2)]
    pub timeout: u8,

    /// (seconds) delay in between repeating requests or batches.
    /// If concurrency is greater than 1 - delay will occur between the batches, not individual URLs
    #[arg(long, short('d'), default_value_t = 0)]
    pub delay: u8,

    /// Verbosity level accumulator, where '-v' some verbosity and '-vvvv' very verbose (trace)
    #[arg(short('v'), action(ArgAction::Count))]
    pub verbosity: u8,

    /// Show per-request report
    #[arg(long("per-request"))]
    pub per_request: bool,

    /// Show supported methods
    #[arg(long("help-methods"))]
    pub help_methods: bool,

    /// Produce sample URLs file
    #[arg(long("help-file"))]
    pub help_file: bool,
}

pub struct Cli {
    pub args: Args,
}

impl Cli {
    pub fn create() -> Result<Cli, u8> {
        let cli = match Cli::from_os_env() {
            Ok(instance) => instance,
            Err(err) => {
                err.print().expect("Unable to format error details");
                return Err(crate::EXIT_ERROR_PARSING_ARGS);
            }
        };
        cli.set_log_level();

        if cli.args.help_methods {
            println!("Supported methods: {}", http::list_methods());
            return Err(0);
        }

        if cli.args.help_file {
            const URL: &str = "http://site.test/url/path?foo=bar";

            println!("# this is comment, it will be ignored as well as empty lines\n\n");
            println!("# following URL will be called with default method: GET");
            println!("{}\n", URL);
            println!("# to get all avaialable methods use '--help-methods' argument");
            for method in http::SUPPORTED_HTTP_METHODS.iter() {
                println!("{} {}", method, URL);
            }
            return Err(0);
        }

        if !http::method_supported(&cli.args.method) {
            println!(
                "Defined method is not supported '{}', try '--help-methods'",
                &cli.args.method
            );
            return Err(crate::EXIT_UNKNOWN_METHOD);
        }

        if cli.args.repeat > 0 && cli.args.delay >= 30 {
            println!(
                "Warning: delay is set to {}s, it seems unreasonably high\n",
                &cli.args.delay
            );
        }

        Ok(cli)
    }

    pub fn from_os_env() -> Result<Cli, clap::error::Error> {
        let args_collection: Vec<OsString> = std::env::args_os().collect();

        Cli::new(args_collection.into_iter())
    }

    /// Create a Cli instance with all the args
    pub fn new(args: IntoIter<OsString>) -> Result<Cli, clap::error::Error> {
        Args::try_parse_from(args).map_or_else(
            |err: clap::error::Error| Err(err),
            |parsed_args| Ok(Cli { args: parsed_args }),
        )
    }

    // set log level based on the CLI args passed
    pub fn set_log_level(&self) {
        let log_level: log::LevelFilter = match &self.args.verbosity {
            1 => log::LevelFilter::Warn,
            2 => log::LevelFilter::Info,
            3 => log::LevelFilter::Debug,
            4.. => log::LevelFilter::Trace,
            _ => log::LevelFilter::Error,
        };
        log_builder::new().filter_level(log_level).init();

        log::warn!("Log level set to {}", log_level);
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsString, vec::IntoIter};

    use crate::components::cli::Cli;

    fn create_iter_from_cmd(cmd: &str) -> IntoIter<OsString> {
        let collection: Vec<OsString> = cmd.split(' ').map(OsString::from).collect();

        collection.into_iter()
    }

    #[test]
    fn test_long_url() {
        let test_args = self::create_iter_from_cmd(
            "program_name.exe -vvv --method TEST123 --concurrent 2 --repeat 3 --timeout 4 \
            --delay 5 http://address.local/long-test",
        );

        let cli = Cli::new(test_args).map_or_else(|err| panic!("{}", err), |instance| instance);

        assert_eq!(cli.args.verbosity, 3);
        assert_eq!(&cli.args.method, "TEST123");
        assert_eq!(cli.args.concurrent, 2);
        assert_eq!(cli.args.repeat, 3);
        assert_eq!(cli.args.timeout, 4);
        assert_eq!(cli.args.delay, 5);

        assert_eq!(&cli.args.url, "http://address.local/long-test");
    }

    #[test]
    fn test_short_url() {
        let test_args = self::create_iter_from_cmd(
            "program_name.exe -vvvv -mTEST123 -c2 -r3 -t4 -d5 http://address.local/short-test",
        );

        let cli = Cli::new(test_args)
            .map_or_else(|_| panic!("Unable to create CLI"), |instance| instance);

        assert_eq!(cli.args.verbosity, 4);
        assert_eq!(&cli.args.method, "TEST123");
        assert_eq!(cli.args.concurrent, 2);
        assert_eq!(cli.args.repeat, 3);
        assert_eq!(cli.args.timeout, 4);
        assert_eq!(cli.args.delay, 5);

        assert_eq!(&cli.args.url, "http://address.local/short-test");
    }

    #[test]
    fn test_short_file() {
        let test_args = self::create_iter_from_cmd("program_name.exe -c3 -r2 -t1 -f filename.txt");

        let cli = Cli::new(test_args).map_or_else(|err| panic!("{}", err), |instance| instance);

        assert_eq!(cli.args.verbosity, 0);
        assert_eq!(&cli.args.method, "GET"); // default
        assert_eq!(cli.args.concurrent, 3);
        assert_eq!(cli.args.repeat, 2);
        assert_eq!(cli.args.timeout, 1);
        assert_eq!(cli.args.delay, 0); // default

        assert_eq!(&cli.args.file, "filename.txt");
    }

    #[test]
    #[should_panic]
    fn test_wrong_values() {
        let test_args =
            self::create_iter_from_cmd("program_name.exe --repeat TWICE http://error.local/");

        Cli::new(test_args).map_or_else(|err| panic!("{}", err), |instance| instance);
    }

    #[test]
    #[should_panic]
    fn test_url_and_file_error() {
        let test_args =
            self::create_iter_from_cmd("program_name.exe --file test.txt http://error.local/");

        Cli::new(test_args).map_or_else(|err| panic!("{}", err), |instance| instance);
    }
}
