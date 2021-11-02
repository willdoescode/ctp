use anyhow::anyhow;
use anyhow::Result;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Cannot execute empty command.")]
    EmptyCommand,
}

pub fn exec(s: &str, proj_name: &str) -> Result<(), anyhow::Error> {
    let replaced = s.replace("{{__NAME__}}", proj_name);
    let split = replaced.split_ascii_whitespace().collect::<Vec<&str>>();

    match split.as_slice() {
        [head, tail @ ..] => execute_command_with_output(head, tail)?,
        [] => return Err(anyhow!(ExecError::EmptyCommand.to_string())),
    }

    Ok(())
}

fn execute_command_with_output(command: &str, args: &[&str]) -> Result<(), anyhow::Error> {
    println!("Executing: {} [{}]", command, &args.to_vec().join(", "));
    Command::new(command).args(args).output()?;

    Ok(())
}
