use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

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
    cmd.assert().success().stdout(contains("10 -1 true false hello world"));

    Ok(())
}
