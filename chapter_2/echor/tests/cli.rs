use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn exit_successfully() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("a b c").assert().success();
    Ok(())
}

fn run(args : &[&str], expected_file : &str ) -> TestResult {
    let expected = std::fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    run(&["Hello there"], outfile)?;
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    run(&["Hello", "there"], outfile)?;
    Ok(())
}
