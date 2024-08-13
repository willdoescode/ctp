mod commands;
mod directory;
mod opts;
mod shape;

use std::io::Read;
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use opts::Opts;

#[cfg(test)]
pub mod tests;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLACEABLE_NAME: &str = "{{__NAME__}}";
pub const REPLACEABLE_OUTPUT: &str = "{{__OUT__}}";

fn read_config_file(config_path: &PathBuf) -> Result<String> {
    let mut config_file = String::new();

    File::open(config_path)
        .with_context(|| format!("Failed to open config file: {}", config_path.display()))?
        .read_to_string(&mut config_file)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    Ok(config_file)
}

fn parse_config(config_file: &str) -> Result<toml::Value> {
    toml::from_str(config_file).context("Failed to parse config file.")
}

fn get_output_path(opts: &Opts) -> Result<&str, anyhow::Error> {
    opts.output
        .as_deref()
        .and_then(|path| path.to_str())
        .ok_or_else(|| anyhow::anyhow!("Output path is not a valid path."))
}

fn run_commands(
    opts: &Opts,
    config: &toml::Value,
    output_path: &str,
    variant: shape::CommandVariants,
) -> Result<()> {
    let variant_clone = variant.clone();

    commands::execute_commands(opts, config, output_path, variant)
        .with_context(|| format!("Failed to execute {:?} commands", variant_clone))
}

fn copy_directory(src: &str, dst: &str, project_name: &str) -> Result<()> {
    directory::copy_dir_recur(src, dst, project_name, dst)
        .with_context(|| format!("Failed to copy directory from {} to {}", src, dst))
}

fn change_directory(path: &str) -> Result<()> {
    std::env::set_current_dir(path)
        .with_context(|| format!("Failed to change current directory to {}", path))
}

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::opts()?;

    let config_file = read_config_file(opts.config.as_ref().unwrap())?;
    let parsed_toml_config = parse_config(&config_file)?;
    let dir_location = shape::get_lang_location(&parsed_toml_config, &opts.language)?;

    let output_path = get_output_path(&opts)?;

    run_commands(
        &opts,
        &parsed_toml_config,
        output_path,
        shape::CommandVariants::Before,
    )?;

    copy_directory(&dir_location, output_path, &opts.project_name)?;

    change_directory(output_path)?;

    run_commands(
        &opts,
        &parsed_toml_config,
        output_path,
        shape::CommandVariants::After,
    )?;

    Ok(())
}
