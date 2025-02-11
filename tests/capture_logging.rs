use escargot::CargoBuild;
use log::LevelFilter;
use rstest::rstest;
use rstest_reuse::{self, *};

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
fn filter_level(
    #[values(
        (LevelFilter::Error, "ERROR"),
        (LevelFilter::Warn, "WARN"),
        (LevelFilter::Info, "INFO"),
        (LevelFilter::Debug, "DEBUG"),
        (LevelFilter::Trace, "TRACE")
    )]
    filter_level: (LevelFilter, &str),
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
#[rstest]
fn stderr_without_level(default_level: LevelFilter) {
    let actual_log = run_cli(default_level, &["--log", "stderr"]);
    assert_eq!(expected_log(default_level), actual_log);
}

#[apply(default_level)]
#[apply(filter_level)]
#[rstest]
fn stderr_with_level(
    #[values(
        LevelFilter::Error,
        LevelFilter::Warn,
        LevelFilter::Info,
        LevelFilter::Debug,
        LevelFilter::Trace
    )]
    default_level: LevelFilter,
    #[values(
        (LevelFilter::Error, "ERROR"),
        (LevelFilter::Warn, "WARN"),
        (LevelFilter::Info, "INFO"),
        (LevelFilter::Debug, "DEBUG"),
        (LevelFilter::Trace, "TRACE")
    )]
    filter_level: (LevelFilter, &str),
) {
    let actual_log = run_cli(
        default_level,
        &["--log", &format!("{}:stderr", filter_level.1)],
    );
    assert_eq!(expected_log(filter_level.0), actual_log);
}
