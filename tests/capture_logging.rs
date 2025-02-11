use assert_fs::{assert::PathAssert as _, prelude::PathChild as _, TempDir};
use escargot::CargoBuild;
use log::LevelFilter;
use predicates::prelude::*;
use rstest::rstest;
use rstest_reuse::{self, *};
use std::path::PathBuf;

#[template]
fn default_level(
    #[values(
        LevelFilter::Error,
        LevelFilter::Warn,
        LevelFilter::Info,
        LevelFilter::Debug,
        LevelFilter::Trace
    )]
    default_level: LevelFilter,
) {
}

#[template]
fn filter_level_1(
    #[values(
        (Some(LevelFilter::Error), "ERROR:"),
        (Some(LevelFilter::Warn), "WARN:"),
        (Some(LevelFilter::Info), "INFO:"),
        (Some(LevelFilter::Debug), "DEBUG:"),
        (Some(LevelFilter::Trace), "TRACE:"),
        (None, "")
    )]
    filter_level_1: (Option<LevelFilter>, &str),
) {
}
#[template]
fn filter_level_2(
    #[values(
        (Some(LevelFilter::Error), "ERROR:"),
        (Some(LevelFilter::Warn), "WARN:"),
        (Some(LevelFilter::Info), "INFO:"),
        (Some(LevelFilter::Debug), "DEBUG:"),
        (Some(LevelFilter::Trace), "TRACE:"),
        (None, "")
    )]
    filter_level_2: (Option<LevelFilter>, &str),
) {
}

fn expected_log(filter_level: LevelFilter) -> String {
    let mut expected_log = String::new();
    if filter_level >= LevelFilter::Trace {
        expected_log += "[TRACE] Some trace log\n";
    }
    if filter_level >= LevelFilter::Debug {
        expected_log += "[DEBUG] Some debug log\n";
    }
    if filter_level >= LevelFilter::Info {
        expected_log += "[INFO] Some info log\n";
    }
    if filter_level >= LevelFilter::Warn {
        expected_log += "[WARN] Some warn log\n";
    }
    if filter_level >= LevelFilter::Error {
        expected_log += "[ERROR] Some error log\n";
    }
    expected_log
}

fn run_cli(default_level: LevelFilter, log_args: &[&str]) -> String {
    let mut args = log_args.to_vec();
    let default_level_str = default_level.to_string();
    args.extend(["--default-level", &default_level_str]);
    let output = CargoBuild::new()
        .example("log_some_lines")
        .run()
        .unwrap()
        .command()
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stderr).unwrap()
}

#[apply(default_level)]
#[apply(filter_level_1)]
#[rstest]
fn stderr(default_level: LevelFilter, filter_level_1: (Option<LevelFilter>, &str)) {
    let actual_log = run_cli(
        default_level,
        &["--log", &format!("{}stderr", filter_level_1.1)],
    );
    let expected_level = filter_level_1.0.unwrap_or(default_level);
    assert_eq!(expected_log(expected_level), actual_log);
}

struct TempLogFile {
    tempdir: TempDir,
}

impl TempLogFile {
    pub fn setup() -> Self {
        let tempdir = TempDir::new().unwrap();
        Self { tempdir }
    }

    pub fn logfile_path(&self) -> PathBuf {
        self.tempdir.path().join("log")
    }

    pub fn assert_was_created_with_content(&self, expected_log: &str) {
        let log_file = self.tempdir.child("log");
        log_file.assert(predicate::path::exists());
        log_file.assert(expected_log);
    }
}

#[apply(default_level)]
#[apply(filter_level_1)]
#[rstest]
fn file(default_level: LevelFilter, filter_level_1: (Option<LevelFilter>, &str)) {
    let logfile = TempLogFile::setup();
    let stderr = run_cli(
        default_level,
        &[
            "--log",
            &format!(
                "{}file:{}",
                filter_level_1.1,
                logfile.logfile_path().display()
            ),
        ],
    );
    let expected_level = filter_level_1.0.unwrap_or(default_level);
    logfile.assert_was_created_with_content(&expected_log(expected_level));
    assert_eq!("", stderr);
}

#[apply(default_level)]
#[apply(filter_level_1)]
#[apply(filter_level_2)]
#[rstest]
fn two_files(
    default_level: LevelFilter,
    filter_level_1: (Option<LevelFilter>, &str),
    filter_level_2: (Option<LevelFilter>, &str),
) {
    let logfile1 = TempLogFile::setup();
    let logfile2 = TempLogFile::setup();
    let stderr = run_cli(
        default_level,
        &[
            "--log",
            &format!(
                "{}file:{}",
                filter_level_1.1,
                logfile1.logfile_path().display()
            ),
            "--log",
            &format!(
                "{}file:{}",
                filter_level_2.1,
                logfile2.logfile_path().display()
            ),
        ],
    );
    let expected_level_1 = filter_level_1.0.unwrap_or(default_level);
    let expected_level_2 = filter_level_2.0.unwrap_or(default_level);
    logfile1.assert_was_created_with_content(&expected_log(expected_level_1));
    logfile2.assert_was_created_with_content(&expected_log(expected_level_2));
    assert_eq!("", stderr);
}

// TODO Tests for logging to syslog
// TODO Tests for disabling logging
// TODO Tests for multiple log destinations
