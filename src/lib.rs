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
//! # Filter log levels. Debug (and higher) gets logged to stderr.
//! # Info (and higher) additionally gets logged to a file.
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
//! # FlagSyntax
//! See [LogArgs](crate::clap::LogArgs) for a detailed explanation of the syntax for the `--log` argument.
//!
//! # Log Format
//! The format of the log messages uses an opinionated, reasonably pretty format by default. At the moment, this library does not offer any way to modify the format.
//!
//! ![Example log file][example_log.png]
//!
//! ```bash
//! [2025-02-14T00:16:14Z $${\color{blue}TRACE}$$ simple_cli] Some trace log
//! [2025-02-14T00:16:14Z DEBUG simple_cli] Some debug log
//! [2025-02-14T00:16:14Z INFO simple_cli] Some info log
//! [2025-02-14T00:16:14Z WARN simple_cli] Some warn log
//! [2025-02-14T00:16:14Z ERROR simple_cli] Some error log
//! ```
//!
//! Log lines contain a UTC timestamp, the log level and an executable name.
//!
//! When the logging destination is stderr and stderr is a terminal, then the log level is colorized.
//! When logging to a file, syslog or to a stderr that is redirected to a file, the log level is not colorized.
//!
#![cfg_attr(all(), doc = ::embed_doc_image::embed_image!("example_log.png", "example_log.png"))]
#![allow(clippy::needless_doctest_main)]
#![forbid(unsafe_code)]
// TODO #![deny(missing_docs)]
// We need to add explicit links because our `gen_readme.sh` script requires them.
#![allow(rustdoc::redundant_explicit_links)]

mod clap;
mod config;
mod fern;
mod parser;

pub use clap::LogArgs;
pub use config::{LogDestination, LogDestinationConfig, LoggingConfig};
pub use fern::_init_logging;
