use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
  return format!("tests/examples/{}.ling", s);
}

#[test]
fn early_return() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("early_return"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("15"));

    Ok(())
}

#[test]
fn fibonacci() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("fibonacci"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("55"));

    Ok(())
}

#[test]
fn identity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("identity"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5true"));

    Ok(())
}

#[test]
fn print_ten() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("print_ten"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("10"));

    Ok(())
}

#[test]
fn short_circuiting() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("short_circuiting"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("true false"));

    Ok(())
}
