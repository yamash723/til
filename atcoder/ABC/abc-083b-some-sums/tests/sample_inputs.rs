use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "84\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "13\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 4 16
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4554\n");
    assert!(output.stderr_str().is_empty());
}

