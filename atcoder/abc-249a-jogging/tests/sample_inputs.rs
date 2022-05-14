use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 3 3 6 2 5 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Takahashi\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 1 4 1 5 9 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Aoki\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 1 1 1 1 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Draw\n");
    assert!(output.stderr_str().is_empty());
}

