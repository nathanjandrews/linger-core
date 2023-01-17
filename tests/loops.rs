use std::process::Command;

use assert_cmd::prelude::*;
use linger_core::error::{ParseError, RuntimeError};
use predicates::{prelude::predicate::str::contains, str::starts_with};

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/loops/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn while_statement() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while"));
    cmd.assert().success().stdout(contains("5 4 3 2 1"));

    Ok(())
}

#[test]
fn while_with_break() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while_with_break"));
    cmd.assert().success().stdout(contains("5 4 3"));

    Ok(())
}

#[test]
fn while_with_continue() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while_with_continue"));
    cmd.assert().success().stdout(contains("10 8 6 4 2"));

    Ok(())
}

#[test]
fn while_with_break_and_continue() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while_with_break_and_continue"));
    cmd.assert().success().stdout(contains("20 18 16 14 12 10"));

    Ok(())
}

#[test]
fn nested_break() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("nested_break"));
    cmd.assert().success().stdout(contains("3 3 \n2 2 \n1 1"));

    Ok(())
}

#[test]
fn nested_continue() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("nested_continue"));
    cmd.assert()
        .success()
        .stdout(contains("4 2 0 \n4 2 0 \n4 2 0 "));

    Ok(())
}

#[test]
fn for_statement() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("for"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}

#[test]
fn for_with_existing_initial_value() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("for_with_existing_initial_value"));
    cmd.assert().success().stdout(contains("55"));

    Ok(())
}

#[test]
fn err_break_not_in_loop() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-break_not_in_loop"));
    cmd.assert()
        .failure()
        .stderr(starts_with(RuntimeError::BreakNotInLoop.to_string()))
        .stdout("");

    Ok(())
}

#[test]
fn err_continue_not_in_loop() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-continue_not_in_loop"));
    cmd.assert()
        .failure()
        .stderr(starts_with(RuntimeError::ContinueNotInLoop.to_string()))
        .stdout("");

    Ok(())
}

#[test]
fn err_expected_update_assignment() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-expected_update_assignment"));
    cmd.assert()
        .failure()
        .stderr(starts_with(ParseError::ExpectedAssignment.to_string()))
        .stdout("");

    Ok(())
}

#[test]
fn err_expected_initial_assign_or_init() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-expected_initial_assign_or_init"));
    cmd.assert()
        .failure()
        .stderr(starts_with(
            ParseError::ExpectedAssignmentOrInitialization.to_string(),
        ))
        .stdout("");

    Ok(())
}
