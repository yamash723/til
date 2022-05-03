use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9 45000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4 0 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 196000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1 -1 -1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1000 1234000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14 27 959\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2000 20000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2000 0 0\n");
    assert!(output.stderr_str().is_empty());
}
