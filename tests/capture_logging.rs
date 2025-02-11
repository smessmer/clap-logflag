use escargot::CargoBuild;

#[test]
fn dummy_integration_test() {
    let output = CargoBuild::new()
        .example("log_some_lines")
        .run()
        .unwrap()
        .command()
        .args(["--log", "stderr"])
        .output()
        .unwrap();
    assert_eq!(
        "[INFO] Some info log\n[WARN] Some warn log\n[ERROR] Some error log\n",
        String::from_utf8(output.stderr).unwrap()
    );
}
