use std::process::Command;

use assert_cmd::prelude::*;
use linger::error::RuntimeError;
use linger::interpreter::Value;
use predicates::{
    prelude::{predicate::str::contains, PredicateBooleanExt},
    str::starts_with,
};

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/operators/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn binary_operators() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("binary_operators"));
    cmd.assert().success().stdout(contains(
        "12 \"hello world\" -2 true false true false true false false true false true true 5 0 35",
    ));

    Ok(())
}

#[test]
fn unary_operators() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("unary_operators"));
    cmd.assert()
        .success()
        .stdout(contains("false true true -10 10"));

    Ok(())
}

#[test]
fn increment_and_decrement() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("increment_and_decrement"));
    cmd.assert()
        .success()
        .stdout(contains("6 7 7 8").and(contains("4 3 3 2")));

    Ok(())
}

#[test]
fn assignment_operators() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("assignment_operators"));
    cmd.assert().success().stdout(contains("15 -5"));

    Ok(())
}

#[test]
fn operator_precedence() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("operator_precedence"));
    cmd.assert()
        .success()
        .stdout(contains("23 1 false true true true"));

    Ok(())
}

#[test]
fn short_circuiting() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("short_circuiting"));
    cmd.assert().success().stdout(contains("true false"));

    Ok(())
}

#[test]
fn err_bad_arg_plus_bool() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-bad_arg_plus_bool"));
    cmd.assert()
        .failure()
        .stderr(starts_with(
            RuntimeError::BadArg(Value::Bool(true)).to_string(),
        ))
        .stdout("");

    Ok(())
}
