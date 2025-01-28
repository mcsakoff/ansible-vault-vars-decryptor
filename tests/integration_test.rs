use std::process::Command;
use std::fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

fn assert_output_contains(output: &str, values: Vec<&'static str>) {
    // Verify the decrypted content contains expected values
    for value in values {
        assert!(output.contains(value));
    }
}

#[test]
fn test_decrypt_known_values() {
    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd.env("ANSIBLE_VAULT_PASSWORD_FILE", "tests/fixtures/vault_password")
        .arg("tests/fixtures/sample-vault-file-test.yml");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_output_contains(&output, vec!["bob", "password123", "supersecret"]);
}

#[test]
fn test_decrypt_nonexistent_file() {
    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd.env("ANSIBLE_VAULT_PASSWORD_FILE", "tests/fixtures/vault_password")
        .arg("tests/fixtures/nonexistent.yml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn test_decrypt_invalid_vault_password() {
    // Create a temporary wrong password file
    let temp_dir = tempdir().unwrap();
    let wrong_pass_file = temp_dir.path().join("wrong_pass");
    fs::write(&wrong_pass_file, "wrongpassword").unwrap();

    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd.env("ANSIBLE_VAULT_PASSWORD_FILE", wrong_pass_file.to_str().unwrap())
        .arg("tests/fixtures/sample-vault-file-test.yml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Decryption failed"));
}

#[test]
fn test_decrypt_multiple_vault_vars() {
    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd.env("ANSIBLE_VAULT_PASSWORD_FILE", "tests/fixtures/vault_password")
        .arg("tests/fixtures/sample-vault-file-test.yml");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    // Verify both vault variables were decrypted
    assert!(!output.contains("$ANSIBLE_VAULT"));

    // Verify the known decrypted values
    // The content should still have the original plain text
    assert_output_contains(&output, vec!["bob", "password123", "supersecret"]);
}

#[test]
fn test_decrypt_executable_password_file() {
    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd
        .env("ANSIBLE_VAULT_PASSWORD_FILE", "tests/fixtures/vault_password_exec")
        .env("VAULT_PASSWORD", "testing123")
        .arg("tests/fixtures/sample-vault-file-test.yml");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    // Verify both vault variables were decrypted
    assert!(!output.contains("$ANSIBLE_VAULT"));

    // Verify the known decrypted values
    // The content should still have the original plain text
    assert_output_contains(&output, vec!["bob", "password123", "supersecret"]);
}

#[test]
fn test_decrypt_with_multiple_identities() {
    let mut cmd = Command::cargo_bin("ansible-decryptor").unwrap();
    cmd
        .env("ANSIBLE_VAULT_IDENTITY_LIST",
             "test1@tests/fixtures/vault_password_exec, test2@tests/fixtures/vault_password")
        .env("VAULT_PASSWORD", "testing12356")  // <-- intentionally incorrect password!
        .arg("tests/fixtures/sample-vault-file-test.yml");

    let assert = cmd.assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    // Verify both vault variables were decrypted
    assert!(!output.contains("$ANSIBLE_VAULT"));

    // Verify the known decrypted values
    // The content should still have the original plain text
    assert_output_contains(&output, vec!["bob", "password123", "supersecret"]);
}
