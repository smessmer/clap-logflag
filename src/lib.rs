//! [work in progress]
//!
//! The [clap-logflag](https://crates.io/crates/clap-logflag) library adds a `--log` flag to clap based applications
//! that allows CLI users to configure logging from the command line.
//! It can log to stderr, files and syslog.
//!
//! # Examples
//! ```bash
//! # Log to a single destination
//! $ ./your-cli --log syslog
//! $ ./your-cli --log file:/path/to/file
//!
//! # Log to both stderr and a file
//! $ ./your-cli --log stderr --log file:/path/to/file
//!
//! # Filter log levels
//! $ ./your-cli --log DEBUG:stderr --log INFO:file:/path/to/file
//!
//! # Disable logging
//! $ ./your-cli --log none
//!
//! # Use default logging setup (defined by the application developer)
//! $ ./your-cli
//! ```
//!
//! # Setup
//! To use clap-logflag, first add [clap-logflag](https://crates.io/crates/clap-logflag), [clap](https://crates.io/crates/clap) and [log](https://crates.io/crates/log) to your `Cargo.toml`.
//!
//! Then, add the [LogArgs](crate::clap::LogArgs) struct to your clap definition and initialize logging with it:
//!
//! ```rust
//! use clap::Parser;
//! use clap_logflag::LoggingConfig;
//! use log::LevelFilter;
//!
//! #[derive(Debug, Parser)]
//! struct CliArgs {
//!     // Use this to add the log flags to your application
//!     #[clap(flatten)]
//!     log: clap_logflag::LogArgs,
//!     
//!     // ... your other cli args ...
//! }
//!
//! fn main() {
//!     let args = CliArgs::parse();
//!
//!     // Initialize logging with the flags from clap
//!     clap_logflag::init_logging!(
//!         args.log
//!             // If no `--log` arguments are present, disable logging.
//!             // You can change this to define the default behavior,
//!             // see the "default_logging" example.
//!             .or_default(LoggingConfig::LoggingDisabled),
//!         // Any `--log` argument that doesn't define a level filter will use the
//!         // default level filter defined here, `Info` in this example.
//!         LevelFilter::Info,
//!     );
//!
//!     // Issue some log messages
//!     log::trace!("Some trace log");
//!     log::debug!("Some debug log");
//!     log::info!("Some info log");
//!     log::warn!("Some warn log");
//!     log::error!("Some error log");
//! }
//! ```
//!
//! # Syntax
//! See [LogArgs](crate::clap::LogArgs) for a detailed explanation of the syntax for the `--log` argument.
//!

#![forbid(unsafe_code)]
// TODO #![deny(missing_docs)]

mod clap;
mod config;
mod fern;
mod parser;

pub use clap::LogArgs;
pub use config::{LogDestination, LogDestinationConfig, LoggingConfig};
pub use fern::_init_logging;
