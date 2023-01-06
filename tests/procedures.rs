use std::process::Command;

use assert_cmd::prelude::*;
use linger::error::ParseError;
use predicates::prelude::predicate::str::contains;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/procedures/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn recursion() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("recursion"));
    cmd.assert()
        .success()
        .stdout(contains("10 9 8 7 6 5 4 3 2 1"));

    Ok(())
}

#[test]
fn closures() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("closures"));
    cmd.assert().success().stdout(contains("10"));

    Ok(())
}

#[test]
fn higher_order_procedure() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("higher_order_procedure"));
    cmd.assert().success().stdout(contains("17 25"));

    Ok(())
}

#[test]
fn err_keyword_as_proc() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-keyword_as_proc"));
    cmd.assert().failure().stderr(contains(
        ParseError::KeywordAsProc("for".to_string()).to_string(),
    ));

    Ok(())
}
