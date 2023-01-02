use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/scope/{}.ling", s);
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
fn scoping() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("scoping"));
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
    cmd.assert().success().stdout(predicate::str::contains("5"));

    Ok(())
}

#[test]
fn block() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("block"));
    cmd.assert().success().stdout(predicate::str::contains("5 6 5"));

    Ok(())
}
