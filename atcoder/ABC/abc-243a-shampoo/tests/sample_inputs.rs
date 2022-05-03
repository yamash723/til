use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"25 10 11 12
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "T\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"30 10 10 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "F\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100000 1 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "M\n");
    assert!(output.stderr_str().is_empty());
}

