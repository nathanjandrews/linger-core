use std::process::Command;

use assert_cmd::prelude::*;
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