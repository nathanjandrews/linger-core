use std::process::Command;

use assert_cmd::prelude::*;
use linger::error::RuntimeError;
use predicates::prelude::predicate::str::contains;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/scope/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn shadowing() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("shadowing"));
    cmd.assert().success().stdout(contains("5 10 5"));

    Ok(())
}

#[test]
fn reassignment_in_block() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("reassignment_in_block"));
    cmd.assert().success().stdout(contains("5 10 10"));

    Ok(())
}

#[test]
fn procedure_shadowing() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("procedure_shadowing"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}

#[test]
fn err_for_loop_var_scope() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-for_loop_var_scope"));
    cmd.assert()
        .failure()
        .stderr(contains(
            RuntimeError::UnknownVariable("a".to_string()).to_string(),
        ))
        .stdout("");

    Ok(())
}

#[test]
fn err_unknown_var() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-unknown_var"));
    cmd.assert()
        .failure()
        .stderr(contains(
            RuntimeError::UnknownVariable("a".to_string()).to_string(),
        ))
        .stdout("");

    Ok(())
}
