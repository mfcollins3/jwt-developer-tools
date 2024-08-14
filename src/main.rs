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

use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rand::Rng;

#[derive(Parser)]
#[command(name = "JWT Developer Tools")]
#[command(version = "0.1.0")]
#[command(about = "Tools to help developers generate and verify JSON web tokens")]
#[command(long_about = "\
JSON Developer Tools is a set of commands that help developers to build
distributed solutions that rely on JSON web tokens for identity and
authorization. Developers can use JSON Developer Tools to generate the
cryptographic keys that are used to sign and verify JSON web tokens, generate
a JSON web token for debugging and testing, verify a JSON web token, or to
view the contents of a JSON web token.")]
struct ProgramArgs {
    /// The path where the secret key should be output
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Hs256,
    Hs384,
    Hs512,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ProgramArgs::parse();
    match args.command {
        Command::Hs256 => generate_hs256_key(args.output),
        Command::Hs384 => generate_hs384_key(args.output),
        Command::Hs512 => generate_hs512_key(args.output),
    }
}

fn generate_hs256_key(output: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    generate_and_output_secret_key(32, output)
}

fn generate_hs384_key(output: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    generate_and_output_secret_key(48, output)
}

fn generate_hs512_key(output: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    generate_and_output_secret_key(64, output)
}

fn generate_and_output_secret_key(
    length: usize,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let secret_key = generate_secret_key(length)?;
    output_secret_key(output, &secret_key)
}

fn generate_secret_key(
    length: usize,
) -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let key = (0..length).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
    let key_string = key.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    Ok(key_string)
}

fn output_secret_key(
    output: Option<PathBuf>,
    key: &String,
) -> Result<(), Box<dyn Error>> {
    let mut writer: Box<dyn std::io::Write> = match output {
        Some(path) => Box::new(std::fs::File::create(path)
            .expect("Unable to create file")),
        None => Box::new(std::io::stdout()),
    };
    writeln!(writer, "{}", key)?;
    Ok(())
}
