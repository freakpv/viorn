use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?;
    let git_hash = String::from_utf8(output.stdout)?;
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    Ok(())
}
