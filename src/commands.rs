use std::process::Command;

use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Cannot execute empty command.")]
    EmptyCommand,
}

pub fn exec(s: &str, proj_name: &str, proj_output: &str) -> Result<(), anyhow::Error> {
    let replaced = s
        .replace(crate::REPLACEABLE_NAME, proj_name)
        .replace(crate::REPLACEABLE_OUTPUT, proj_output);
    let split = replaced.split_ascii_whitespace().collect::<Vec<&str>>();

    match split.as_slice() {
        [head, tail @ ..] => execute_command_with_output(head, tail)?,
        [] => return Err(anyhow!(ExecError::EmptyCommand.to_string())),
    }

    Ok(())
}

fn execute_command_with_output(command: &str, args: &[&str]) -> Result<(), anyhow::Error> {
    println!("[CMD]    {} [{}]", command, &args.to_vec().join(", "));
    let cmd = Command::new(command).args(args).output()?;
    println!("[STDOUT] {}", std::str::from_utf8(cmd.stdout.as_slice())?);

    Ok(())
}
