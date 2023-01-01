use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/loops/{}.ling", s);
}

#[test]
fn while_statement() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5 4 3 2 1"));

    Ok(())
}

#[test]
fn while_with_break() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while_with_break"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5 4 3 3"));

    Ok(())
}

#[test]
fn nested_while_with_break() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("nested_while_with_break"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("35 34"));

    Ok(())
}

#[test]
fn continue_statement() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("continue"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("9 7 5 3 1"));

    Ok(())
}

#[test]
fn while_with_continue_and_break() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("while_with_continue_and_break"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("9 7 5"));

    Ok(())
}
