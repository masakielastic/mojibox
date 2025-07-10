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

// Tests for len command
#[test]
fn test_len_grapheme_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_len_grapheme_japanese() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("--mode")
        .arg("grapheme")
        .arg("あいうえお")
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_len_codepoint_emoji() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("--mode")
        .arg("codepoint")
        .arg("🍣🍺")
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_len_byte_mode() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("--mode")
        .arg("byte")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_len_empty_string() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

// Tests for take command
#[test]
fn test_take_grapheme_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("3")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\nl\n"));
}

#[test]
fn test_take_grapheme_japanese() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("--mode")
        .arg("grapheme")
        .arg("2")
        .arg("あいうえお")
        .assert()
        .success()
        .stdout(predicate::str::contains("あ\nい\n"));
}

#[test]
fn test_take_codepoint_emoji() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("--mode")
        .arg("codepoint")
        .arg("1")
        .arg("🍣🍺")
        .assert()
        .success()
        .stdout(predicate::str::contains("🍣\n"));
}

#[test]
fn test_take_byte_mode() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("--mode")
        .arg("byte")
        .arg("2")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("h\ne\n"));
}

#[test]
fn test_take_n_greater_than_length() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("10")
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::contains("a\nb\nc\n"));
}

#[test]
fn test_take_zero() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("0")
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

// Tests for drop command
#[test]
fn test_drop_grapheme_basic() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("2")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("l\nl\no\n"));
}

#[test]
fn test_drop_grapheme_japanese() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("--mode")
        .arg("grapheme")
        .arg("2")
        .arg("あいうえお")
        .assert()
        .success()
        .stdout(predicate::str::contains("う\nえ\nお\n"));
}

#[test]
fn test_drop_codepoint_emoji() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("--mode")
        .arg("codepoint")
        .arg("1")
        .arg("🍣🍺")
        .assert()
        .success()
        .stdout(predicate::str::contains("🍺\n"));
}

#[test]
fn test_drop_byte_mode() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("--mode")
        .arg("byte")
        .arg("1")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("e\nl\nl\no\n"));
}

#[test]
fn test_drop_n_greater_than_length() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("10")
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_drop_zero() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("0")
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::contains("a\nb\nc\n"));
}

// Complex Unicode tests
#[test]
fn test_complex_emoji_len() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("len")
        .arg("--mode")
        .arg("grapheme")
        .arg("👨‍💻👩‍🍳")
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_complex_emoji_take() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("take")
        .arg("--mode")
        .arg("grapheme")
        .arg("1")
        .arg("👨‍💻👩‍🍳")
        .assert()
        .success()
        .stdout(predicate::str::contains("👨‍💻\n"));
}

#[test]
fn test_complex_emoji_drop() {
    let mut cmd = Command::cargo_bin("mojibox").unwrap();
    cmd.arg("drop")
        .arg("--mode")
        .arg("grapheme")
        .arg("1")
        .arg("👨‍💻👩‍🍳")
        .assert()
        .success()
        .stdout(predicate::str::contains("👩‍🍳\n"));
}
