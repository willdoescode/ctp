use std::process::Command;

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Cannot execute empty command.")]
    EmptyCommand,
}

pub fn execute_commands(
    opts: &crate::opts::Opts,
    toml_config: &toml::Value,
    output_path: &str,
    command_variant: crate::shape::CommandVariants,
) -> Result<()> {
    if let Some(commands) =
        crate::shape::get_commands(toml_config, &opts.language, command_variant)?
    {
        for command in commands {
            exec(&command, &opts.project_name, &output_path)?
        }
    }

    Ok(())
}

pub fn exec(s: &str, proj_name: &str, proj_output: &str) -> Result<(), anyhow::Error> {
    let replaced = s
        .replace(crate::REPLACEABLE_NAME, proj_name)
        .replace(crate::REPLACEABLE_OUTPUT, proj_output);

    let mut parts = replaced.split_ascii_whitespace();
    let command = parts.next().ok_or_else(|| ExecError::EmptyCommand)?;
    let args: Vec<&str> = parts.collect();

    execute_command_with_output(command, &args)
}

fn execute_command_with_output(command: &str, args: &[&str]) -> Result<(), anyhow::Error> {
    println!("[CMD]    {} {}", command, args.join(" "));

    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {} {}", command, args.join(" ")))?;

    if !output.stdout.is_empty() {
        println!("[STDOUT] {}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        println!("[STDERR] {}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        return Err(anyhow!("Command failed: {}", output.status));
    }

    Ok(())
}
