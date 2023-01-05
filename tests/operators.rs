use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/operators/{}.ling", s);
}

#[test]
fn logical_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("logical_operators"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("false true"));

    Ok(())
}

#[test]
fn relational_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("relational_operators"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("true false true true false false"));

    Ok(())
}

#[test]
fn numerical_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("numerical_operators"));
    cmd.assert()
        .success()
        // numbers are currently only integers so division is actual integer division
        .stdout(predicate::str::contains("2 -5 12 17 7 4 1"));

    Ok(())
}

#[test]
fn unary_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("unary_operators"));
    cmd.assert()
        .success()
        // numbers are currently only integers so division is actual integer division
        .stdout(predicate::str::contains("false true -5 -10 15"));

    Ok(())
}

#[test]
fn inc_and_dec_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("inc_dec_operators"));
    cmd.assert()
        .success()
        // numbers are currently only integers so division is actual integer division
        .stdout(predicate::str::contains("11 12 12 13").and(predicate::str::contains("9 8 8 7")));

    Ok(())
}

#[test]
fn assignment_operators() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("assignment_operators"));
    cmd.assert()
        .success()
        // numbers are currently only integers so division is actual integer division
        .stdout(predicate::str::contains("15"));

    Ok(())
}
