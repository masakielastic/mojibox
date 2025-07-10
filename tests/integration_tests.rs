use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_iter_grapheme_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\nl\no\n"));
}

#[test]
fn test_iter_grapheme_japanese() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("あいうえお")
        .assert()
        .success()
        .stdout(predicate::str::contains("あ\nい\nう\nえ\nお\n"));
}

#[test]
fn test_iter_grapheme_emoji() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("🍣🍺")
        .assert()
        .success()
        .stdout(predicate::str::contains("🍣\n🍺\n"));
}

#[test]
fn test_iter_grapheme_mixed() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("あいうえお🍣🍺")
        .assert()
        .success()
        .stdout(predicate::str::contains("あ\nい\nう\nえ\nお\n🍣\n🍺\n"));
}

#[test]
fn test_iter_codepoint_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("codepoint")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\nl\no\n"));
}

#[test]
fn test_iter_codepoint_japanese() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("codepoint")
        .arg("あいう")
        .assert()
        .success()
        .stdout(predicate::str::contains("あ\nい\nう\n"));
}

#[test]
fn test_iter_byte_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("byte")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\nl\no\n"));
}

#[test]
fn test_iter_empty_string() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_iter_default_mode() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\nl\no\n"));
}

#[test]
fn test_iter_default_engine() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("iter")
        .arg("--mode")
        .arg("grapheme")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\nl\no\n"));
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A CLI tool for flexible Unicode string manipulation",
        ));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mojibox"));
}
