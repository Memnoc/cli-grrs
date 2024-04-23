use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn find_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = assert_fs::NamedTempFile::new("empty_sample.txt")?;
    temp_file.write_str("")?;
    println!("Path: {:?}", temp_file.path());

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(temp_file.path());
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n"); // byte string literal &[u8]
}
