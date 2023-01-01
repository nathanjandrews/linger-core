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
fn scoping() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("scoping"));
    cmd.assert().success().stdout(predicate::str::contains("6"));

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

#[test]
fn closures() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("closures"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("10"));

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
fn else_if_with_else() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("else_if_with_else"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("branch 2"));

    Ok(())
}

#[test]
fn else_if_no_else() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("else_if_no_else"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("branch 2"));

    Ok(())
}

#[test]
fn else_if_no_else_all_false() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("else_if_no_else_all_false"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5"));

    Ok(())
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
