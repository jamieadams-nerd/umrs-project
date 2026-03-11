use anyhow::{Context, Result, bail};
use std::env;
use std::process::{Command, Stdio};

fn run(cmd: &mut Command) -> Result<()> {
    eprintln!("[xtask] {:?}", cmd);
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to spawn command")?;

    if !status.success() {
        bail!("command failed: {:?}", cmd);
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let sub = args.next().unwrap_or_else(|| "help".to_string());

    match sub.as_str() {
        "fmt" => {
            run(Command::new("cargo").arg("fmt").arg("--all"))?;
        }
        "clippy" => {
            run(Command::new("cargo")
                .arg("clippy")
                .arg("--workspace")
                .arg("--all-targets")
                .arg("--")
                .arg("-D")
                .arg("warnings"))?;
        }
        "test" => {
            run(Command::new("cargo").arg("test").arg("--workspace"))?;
        }
        "robots" => {
            // Example: run your robot generator crate (adjust args as needed)
            run(Command::new("cargo").args(["run", "-p", "robotgen", "--"]))?;
        }
        "help" | "-h" | "--help" => {
            eprintln!(
                "Usage: cargo xtask <cmd>\n\
                 \n\
                 Commands:\n\
                 \tfmt\n\
                 \tclippy\n\
                 \ttest\n\
                 \trobots\n"
            );
        }
        other => bail!("unknown xtask command: {other}"),
    }

    Ok(())
}
