use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

fn file_name_to_path(s: &str) -> String {
    return format!("test_programs/miscellaneous/{}.ling", s);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn comments() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("comments"));
    cmd.assert().success().stdout(contains("success"));

    Ok(())
}
