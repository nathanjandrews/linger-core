use std::process::Command;

use assert_cmd::prelude::*;
use linger::error::{ParseError, RuntimeError};
use predicates::prelude::predicate::str::{contains, starts_with};

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/builtins/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn is_empty() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_empty_true"));
    cmd.assert().success().stdout(starts_with("true"));

    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_empty_false"));
    cmd.assert().success().stdout(starts_with("false"));

    Ok(())
}

#[test]
fn is_nil() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_nil_true"));
    cmd.assert().success().stdout(starts_with("true"));

    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_nil_false"));
    cmd.assert().success().stdout(contains("false").count(4));

    Ok(())
}

#[test]
fn err_is_empty_non_list() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-is_empty_non_list"));
    cmd.assert()
        .failure()
        .stderr(starts_with(
            RuntimeError::ExpectedList("true".to_string()).to_string(),
        ))
        .stdout("");

    Ok(())
}
