mod commands;
mod directory;
mod opts;
mod shape;

use std::fs::File;
use std::io::Read;

use anyhow::Result;
use opts::Opts;

#[cfg(test)]
pub mod tests;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_PROJECT_LOCATION: &str = "_";

pub const REPLACEABLE_NAME: &str = "{{__NAME__}}";
pub const REPLACEABLE_OUTPUT: &str = "{{__OUT__}}";

fn get_config_file(opts: &Opts) -> Result<String> {
    let mut config_file = String::new();
    File::open(&opts.config)?.read_to_string(&mut config_file)?;
    Ok(config_file)
}

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::opts()?;

    let config_file = get_config_file(&opts)?;
    let parsed_toml_config = toml::from_str(&config_file)?;
    let dir_location = shape::get_lang_location(&parsed_toml_config, &opts.language)?;
    let output_path = &opts.output.as_path().to_str().unwrap();

    commands::execute_commands(
        &opts,
        &parsed_toml_config,
        output_path,
        shape::CommandVariants::Before,
    )?;

    directory::copy_dir_all(
        &dir_location,
        &opts.output,
        &opts.project_name,
        &output_path,
    )?;

    std::env::set_current_dir(&opts.output)?;

    commands::execute_commands(
        &opts,
        &parsed_toml_config,
        output_path,
        shape::CommandVariants::After,
    )?;

    Ok(())
}
