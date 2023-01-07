use std::process::Command;

use assert_cmd::prelude::*;
use linger::error::{ParseError, RuntimeError};
use predicates::prelude::predicate::str::contains;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/assignment/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn initialization() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("initialization"));
    cmd.assert().success();

    Ok(())
}

#[test]
fn reassignment() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("reassignment"));
    cmd.assert().success();

    Ok(())
}

#[test]
fn err_keyword_as_var() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-keyword_as_var"));
    cmd.assert().failure().stderr(contains(
        ParseError::KeywordAsVar("true".to_string()).to_string(),
    ));

    Ok(())
}

#[test]
fn err_invalid_assignment_target() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-invalid_assignment_target"));
    cmd.assert().failure().stderr(contains(
        RuntimeError::InvalidAssignmentTarget.to_string(),
    ));

    Ok(())
}
