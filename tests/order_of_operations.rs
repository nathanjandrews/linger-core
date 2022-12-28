use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
  return format!("tests/examples/{}.ling", s);
}

#[test]
fn logical_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("logical_operators"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("falsetrue"));

    Ok(())
}

#[test]
fn relational_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("relational_operators"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("truefalsetruetruefalse"));

    Ok(())
}

#[test]
fn numerical_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("numerical_operators"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2 -5 12 17"));

    Ok(())
}