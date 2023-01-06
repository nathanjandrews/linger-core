use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/control_flow/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn if_else_flow() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("if_else"));
    cmd.assert().success().stdout(contains("success success"));

    Ok(())
}

#[test]
fn if_no_else() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("if_no_else"));
    cmd.assert().success().stdout(contains("success success"));

    Ok(())
}

#[test]
fn multi_branch_conditional() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("multi_branch"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}

#[test]
fn multi_branch_conditional_with_else() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("multi_branch_else"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}

#[test]
fn early_return() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("early_return"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}

#[test]
fn return_from_block() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("return_from_block"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}
