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

#[test]
fn string_concatenation() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("string_concatenation"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world"));

    Ok(())
}

#[test]
fn summing_integers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("summing_integers"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("recursive sum: 55, algebraic sum: 55"));

    Ok(())
}

#[test]
fn scoping() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("scoping"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("6"));

    Ok(())
}

#[test]
fn reassignment() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("reassignment"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("6"));

    Ok(())
}

#[test]
fn shadowing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("shadowing"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("565"));

    Ok(())
}

#[test]
fn reassignment_and_shadowing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("reassignment_and_shadowing"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5 10|9 21|5 21"));

    Ok(())
}

#[test]
fn escape_sequences() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("escape_sequences"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("first line\nsecond line"));

    Ok(())
}

#[test]
fn lambdas() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("lambdas"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("20"));

    Ok(())
}

#[test]
fn procedure_shadowing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("procedure_shadowing"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5 6|5 5"));

    Ok(())
}
