// Copyright 2024 Michael F. Collins, III
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::io::Read;
use std::process::Command;

#[test]
fn hs512_secret_key_is_64_hex_characters() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("jwt")?;
    cmd.arg("hs512");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("^[0-9a-fA-F]{128}\\n$")?);
    Ok(())
}

#[test]
fn hs512_secret_key_is_written_to_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("jwt")?;
    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_file_path = temp_file.into_temp_path();
    cmd.arg("--output")
        .arg(temp_file_path.to_path_buf())
        .arg("hs512");
    cmd.assert().success();
    let mut file = std::fs::File::open(temp_file_path)?;
    let mut key = String::new();
    let length = file.read_to_string(&mut key)?;
    assert_eq!(length, 129);
    let re = regex::Regex::new("^[0-9a-fA-F]{128}\\n$")?;
    assert!(re.is_match(key.as_str()), "The key is not a valid hexadecimal string");
    Ok(())
}