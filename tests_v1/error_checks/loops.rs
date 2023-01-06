use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/error_checks/loops/{}.ling", s);
}

#[test]
fn break_not_in_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("break_not_in_loop"));
    cmd.assert().success().stdout(predicate::str::contains(
        "tried to break while not within a loop",
    ));

    Ok(())
}

#[test]
fn for_locally_initialized_var() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("for_locally_initialized_var"));
    cmd.assert().success().stdout(
        predicate::str::contains("0 1 2 3 4 5 6 7 8 9")
            .and(predicate::str::contains("unknown variable \"a\"")),
    );

    Ok(())
}

#[test]
fn for_unknown_var() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("for_unknown_var"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("unknown variable \"a\""));

    Ok(())
}
