use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/{}.ling", s);
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
fn reassignment() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("reassignment"));
    cmd.assert().success().stdout(predicate::str::contains("6"));

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
fn even_and_odd() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("even_and_odd"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("true"));

    Ok(())
}

#[test]
fn comments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("comments"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello // world"));

    Ok(())
}
