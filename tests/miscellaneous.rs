use std::process::Command;

use assert_cmd::prelude::*;
use linger::{error::{ParseError, TokenizerError, RuntimeError}, interpreter::Value};
use predicates::prelude::predicate::str::{contains, starts_with};

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

#[test]
fn string_indexing() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("string_indexing"));
    cmd.assert().success().stdout(starts_with("n"));

    Ok(())
}

#[test]
fn is_empty() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_empty_true"));
    cmd.assert().success().stdout(starts_with("true"));

    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_empty_false"));
    cmd.assert().success().stdout(starts_with("false"));

    Ok(())
}

#[test]
fn is_nil() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_nil_true"));
    cmd.assert().success().stdout(starts_with("true"));

    let mut cmd = Command::cargo_bin("linger")?;
    cmd.arg(file_name_to_path("is_nil_false"));
    cmd.assert().success().stdout(contains("false").count(4));

    Ok(())
}

#[test]
fn err_is_empty_non_list() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-is_empty_non_list"));
    cmd.assert()
        .failure()
        .stderr(starts_with(
            RuntimeError::ExpectedList(Value::Bool(true)).to_string(),
        ))
        .stdout("");

    Ok(())
}

#[test]
fn err_missing_main() -> TestResult {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-missing_main"));
    cmd.assert()
        .failure()
        .stderr(starts_with(ParseError::NoMain.to_string()));

    Ok(())
}

#[test]
fn err_multiple_top_level_procs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-multiple_top_level_procs"));
    cmd.assert().failure().stderr(starts_with(
        ParseError::MultipleSameNamedProcs("main".to_string()).to_string(),
    ));

    Ok(())
}

#[test]
fn err_invalid_escape_sequence() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-invalid_escape_sequence"));
    cmd.assert().failure().stderr(starts_with(
        TokenizerError::InvalidEscapeSequence('f').to_string(),
    ));

    Ok(())
}

#[test]
fn err_missing_semicolon() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-missing_semicolon"));
    cmd.assert()
        .failure()
        .stderr(starts_with("expected token \";\""));

    Ok(())
}

#[test]
fn err_unterminated_string_literal() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-unterminated_string_literal"));
    cmd.assert()
        .failure()
        .stderr(starts_with("unterminated string literal"))
        .stdout("");

    Ok(())
}

#[test]
fn err_unexpected_eof() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("err-unexpected_eof"));
    cmd.assert()
        .failure()
        .stderr(starts_with("unexpected end of file"))
        .stdout("");

    Ok(())
}
