// tests/integration_test.rs
use std::process::Command;

fn run_test(file_path: &str, expected_output: &str) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(file_path)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), expected_output);
}
#[test]
fn test_step1_valid() {
    run_test("tests/step1/valid.json", "valid");
}

#[test]
fn test_step1_invalid() {
    run_test("tests/step1/invalid.json", "invalid");
}

#[test]
fn test_step2_valid() {
    run_test("tests/step2/valid.json", "valid");
}

#[test]
fn test_step2_invalid() {
    run_test("tests/step2/invalid.json", "invalid");
}

#[test]
fn test_step2_valid2() {
    run_test("tests/step2/valid2.json", "valid");
}

#[test]
fn test_step2_invalid2() {
    run_test("tests/step2/invalid2.json", "invalid");
}

#[test]
fn test_step3_valid() {
    run_test("tests/step3/valid.json", "valid");
}

#[test]
fn test_step3_invalid() {
    run_test("tests/step3/invalid.json", "invalid");
}

#[test]
fn test_step4_valid() {
    run_test("tests/step4/valid.json", "valid");
}

#[test]
fn test_step4_invalid() {
    run_test("tests/step4/invalid.json", "invalid");
}

#[test]
fn test_step4_valid2() {
    run_test("tests/step4/valid2.json", "valid");
}
