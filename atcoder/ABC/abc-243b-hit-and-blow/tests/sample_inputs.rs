use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 3 5 2
2 3 1 4     
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1
2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2 3
4 5 6            
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0
0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7
4 8 1 7 9 5 6
3 5 1 7 8 2 6        
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3
2\n");
    assert!(output.stderr_str().is_empty());
}

