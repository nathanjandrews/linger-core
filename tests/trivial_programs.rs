use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::{contains, starts_with};

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/trivial_programs/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn empty_program() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("empty_program"));
    cmd.assert().success();

    Ok(())
}

#[test]
fn basic_values() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("basic_values"));
    cmd.assert()
        .success()
        .stdout(contains("nil 10 -1 3.14 -10.24 true false hello world"));

    Ok(())
}

#[test]
fn empty_block() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("empty_block"));
    cmd.assert().success();

    Ok(())
}

#[test]
fn err_malformed_decimal() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-malformed_decimal"));
    cmd.assert()
        .failure()
        .stderr(starts_with("unexpected token \".\""));

    Ok(())
}
