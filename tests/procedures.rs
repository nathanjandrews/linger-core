use std::process::Command;

use assert_cmd::prelude::*;
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
