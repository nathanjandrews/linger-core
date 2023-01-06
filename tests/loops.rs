use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

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
    cmd.assert()
        .success()
        .stdout(contains("success"));

    Ok(())
}

#[test]
fn for_with_existing_initial_value() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("for_with_existing_initial_value"));
    cmd.assert()
        .success()
        .stdout(contains("55"));

    Ok(())
}