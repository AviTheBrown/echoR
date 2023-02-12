use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echoR").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn hello1() -> TestResult {
    let expected =
    fs::read_to_string("tests/expected/hello.txt")?;
    let mut cmd = Command::cargo_bin("echoR").unwrap();
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
    OK(())
}

#[test]
fn hello2() -> TestResult {
    let expected =
        fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    OK(())
}