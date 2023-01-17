use std::process::Command;

use assert_cmd::prelude::*;
use linger::{error::RuntimeError, interpreter::Value};
use predicates::prelude::predicate::str::starts_with;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/lists/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn list_initialization() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("list_initialization"));
    cmd.assert().success().stdout(starts_with("[1, 2, [4, 5]]"));

    Ok(())
}

#[test]
fn list_indexing() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("list_indexing"));
    cmd.assert().success().stdout(starts_with("1"));

    Ok(())
}

#[test]
fn list_concatenation() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("list_concatenation"));
    cmd.assert()
        .success()
        .stdout(starts_with("[[1, 2, 3], [4, 5, 6], [1, 2, 3, 4, 5, 6]]"));

    Ok(())
}

#[test]
fn head() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("head"));
    cmd.assert().success().stdout(starts_with("1 nil"));

    Ok(())
}

#[test]
fn rest() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("rest"));
    cmd.assert().success().stdout(starts_with("[2, 3] nil"));

    Ok(())
}

#[test]
fn err_head_non_list() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-head_non_list"));
    cmd.assert().failure().stdout("").stderr(starts_with(
        RuntimeError::ExpectedList(Value::Num(4.0)).to_string(),
    ));

    Ok(())
}

#[test]
fn err_rest_non_list() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-rest_non_list"));
    cmd.assert().failure().stdout("").stderr(starts_with(
        RuntimeError::ExpectedList(Value::Nil).to_string(),
    ));

    Ok(())
}

#[test]
fn err_indexing_non_list() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-indexing_non_list"));
    cmd.assert().failure().stdout("").stderr(starts_with(
        RuntimeError::NotIndexable(Value::Num(10.0)).to_string(),
    ));

    Ok(())
}

#[test]
fn err_index_out_of_bounds() -> TestResult {
    let mut cmd_higher = Command::cargo_bin("linger")?;

    cmd_higher.arg(file_name_to_path("err-index_out_of_bounds_higher"));
    cmd_higher
        .assert()
        .failure()
        .stdout("")
        .stderr(starts_with(RuntimeError::IndexOutOfBounds(3).to_string()));

    let mut cmd_lower = Command::cargo_bin("linger")?;

    cmd_lower.arg(file_name_to_path("err-index_out_of_bounds_lower"));
    cmd_lower
        .assert()
        .failure()
        .stdout("")
        .stderr(starts_with(RuntimeError::IndexOutOfBounds(-1).to_string()));

    Ok(())
}

#[test]
fn err_index_not_an_integer() -> TestResult {
    let mut cmd_string = Command::cargo_bin("linger")?;

    cmd_string.arg(file_name_to_path("err-index_not_an_integer_string"));
    cmd_string.assert().failure().stdout("").stderr(starts_with(
        RuntimeError::ExpectedInteger(Value::Str("hello".to_string())).to_string(),
    ));

    let mut cmd_float = Command::cargo_bin("linger")?;

    cmd_float.arg(file_name_to_path("err-index_not_an_integer_float"));
    cmd_float.assert().failure().stdout("").stderr(starts_with(
        RuntimeError::ExpectedInteger(Value::Num(3.14)).to_string(),
    ));

    Ok(())
}
