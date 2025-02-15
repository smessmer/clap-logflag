use assert_fs::{assert::PathAssert as _, fixture::ChildPath, prelude::PathChild as _, TempDir};
use escargot::CargoBuild;
use log::LevelFilter;
use predicates::prelude::*;
use rstest::rstest;
use rstest_reuse::{self, *};
use std::fmt::Write;
use std::path::{Path, PathBuf};

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

fn expected_log_regex(filter_level: LevelFilter) -> String {
    let timestamp_regex = r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)";
    let mut expected_log = "^".to_string();
    if filter_level >= LevelFilter::Trace {
        write!(
            expected_log,
            r"\[{timestamp_regex} TRACE integration_test\] Some trace log\n"
        )
        .unwrap();
    }
    if filter_level >= LevelFilter::Debug {
        write!(
            expected_log,
            r"\[{timestamp_regex} DEBUG integration_test\] Some debug log\n"
        )
        .unwrap();
    }
    if filter_level >= LevelFilter::Info {
        write!(
            expected_log,
            r"\[{timestamp_regex} INFO integration_test\] Some info log\n"
        )
        .unwrap();
    }
    if filter_level >= LevelFilter::Warn {
        write!(
            expected_log,
            r"\[{timestamp_regex} WARN integration_test\] Some warn log\n"
        )
        .unwrap();
    }
    if filter_level >= LevelFilter::Error {
        write!(
            expected_log,
            r"\[{timestamp_regex} ERROR integration_test\] Some error log\n"
        )
        .unwrap();
    }
    write!(expected_log, "$").unwrap();
    expected_log
}

fn run_cli(default_level: LevelFilter, log_args: &[&str]) -> String {
    let mut args = log_args.to_vec();
    let default_level_str = default_level.to_string();
    args.extend(["--default-level", &default_level_str]);
    let output = CargoBuild::new()
        .example("integration_test")
        .current_release()
        .current_target()
        .run()
        .unwrap()
        .command()
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stderr).unwrap()
}

fn log_arg_stderr(level: &str) -> String {
    format!("{}stderr", level)
}

fn log_arg_file(level: &str, path: &Path) -> String {
    format!("{}file:{}", level, path.display())
}

#[apply(default_level)]
#[apply(filter_level_1)]
#[rstest]
fn stderr(default_level: LevelFilter, filter_level_1: (Option<LevelFilter>, &str)) {
    let actual_log = run_cli(default_level, &["--log", &log_arg_stderr(filter_level_1.1)]);
    let expected_level = filter_level_1.0.unwrap_or(default_level);
    let expected_log_regex = expected_log_regex(expected_level);
    assert!(predicates::str::is_match(expected_log_regex)
        .unwrap()
        .eval(&actual_log));
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
        self.logfile().path().to_path_buf()
    }

    pub fn assert_was_created_with_content(&self, expected_log_regex: &str) {
        let log_file = self.logfile();
        log_file.assert(predicate::path::exists());
        log_file.assert(predicate::str::is_match(expected_log_regex).unwrap());
    }

    fn logfile(&self) -> ChildPath {
        self.tempdir.child("logfile")
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
            &log_arg_file(filter_level_1.1, &logfile.logfile_path()),
        ],
    );
    let expected_level = filter_level_1.0.unwrap_or(default_level);
    logfile.assert_was_created_with_content(&expected_log_regex(expected_level));
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
            &log_arg_file(filter_level_1.1, &logfile1.logfile_path()),
            "--log",
            &log_arg_file(filter_level_2.1, &logfile2.logfile_path()),
        ],
    );
    let expected_level_1 = filter_level_1.0.unwrap_or(default_level);
    let expected_level_2 = filter_level_2.0.unwrap_or(default_level);
    logfile1.assert_was_created_with_content(&expected_log_regex(expected_level_1));
    logfile2.assert_was_created_with_content(&expected_log_regex(expected_level_2));
    assert_eq!("", stderr);
}

#[apply(default_level)]
#[apply(filter_level_1)]
#[apply(filter_level_2)]
#[rstest]
fn file_and_stderr(
    default_level: LevelFilter,
    filter_level_1: (Option<LevelFilter>, &str),
    filter_level_2: (Option<LevelFilter>, &str),
) {
    let logfile = TempLogFile::setup();
    let stderr = run_cli(
        default_level,
        &[
            "--log",
            &log_arg_file(filter_level_1.1, &logfile.logfile_path()),
            "--log",
            &log_arg_stderr(filter_level_2.1),
        ],
    );
    let expected_level_1 = filter_level_1.0.unwrap_or(default_level);
    let expected_level_2 = filter_level_2.0.unwrap_or(default_level);
    logfile.assert_was_created_with_content(&expected_log_regex(expected_level_1));
    assert!(
        predicates::str::is_match(expected_log_regex(expected_level_2))
            .unwrap()
            .eval(&stderr)
    );
}

#[rstest]
fn no_flag_uses_default_logging() {
    // The test binary default logging means log to stderr, but only WARN and ERROR.

    let stderr = run_cli(LevelFilter::Info, &[]);
    assert!(
        predicates::str::is_match(expected_log_regex(LevelFilter::Warn))
            .unwrap()
            .eval(&stderr)
    );
}

#[rstest]
fn disable_logging() {
    let stderr = run_cli(LevelFilter::Info, &["--log", "none"]);
    assert_eq!("", stderr);
}

#[rstest]
fn disable_logging_with_two_nones() {
    let stderr = run_cli(LevelFilter::Info, &["--log", "none", "--log", "none"]);
    assert_eq!("", stderr);
}

#[rstest]
fn none_destination_doesnt_disable_if_other_destination_is_present() {
    let stderr = run_cli(LevelFilter::Info, &["--log", "none", "--log", "stderr"]);
    assert!(
        predicates::str::is_match(expected_log_regex(LevelFilter::Info))
            .unwrap()
            .eval(&stderr)
    );
}

#[rstest]
fn levels_are_case_insensitive(
    #[values(
        (LevelFilter::Error, "ERROR:"),
        (LevelFilter::Error, "error:"),
        (LevelFilter::Error, "ErRoR:"),
        (LevelFilter::Warn, "WARN:"),
        (LevelFilter::Warn, "warn:"),
        (LevelFilter::Warn, "WaRn:"),
        (LevelFilter::Info, "INFO:"),
        (LevelFilter::Info, "info:"),
        (LevelFilter::Info, "InFo:"),
        (LevelFilter::Debug, "DEBUG:"),
        (LevelFilter::Debug, "debug:"),
        (LevelFilter::Debug, "DeBuG:"),
        (LevelFilter::Trace, "TRACE:"),
        (LevelFilter::Trace, "trace:"),
        (LevelFilter::Trace, "TrAcE:")
    )]
    filter_level: (LevelFilter, &str),
) {
    let stderr = run_cli(
        LevelFilter::Info,
        &["--log", &log_arg_stderr(filter_level.1)],
    );
    let expected_level = filter_level.0;
    assert!(
        predicates::str::is_match(expected_log_regex(expected_level))
            .unwrap()
            .eval(&stderr)
    );
}

// TODO Tests for logging to syslog
