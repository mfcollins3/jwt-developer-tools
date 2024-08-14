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

use std::path::PathBuf;

use clap::Parser;
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ProgramArgs::parse();

    let mut writer: Box<dyn std::io::Write> = match args.output {
        Some(path) => Box::new(std::fs::File::create(path).expect("Unable to create file")),
        None => Box::new(std::io::stdout()),
    };
    let mut rng = rand::thread_rng();
    let key = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
    for byte in key {
        write!(writer, "{:02x}", byte)?;
    }
    
    writeln!(writer)?;
    Ok(())
}
