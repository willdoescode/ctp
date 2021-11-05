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

pub const REPLACEABLE_NAME: &str = "{{__NAME__}}";
pub const REPLACEABLE_OUTPUT: &str = "{{__OUT__}}";

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::opts()?;
    let mut config_file = String::new();
    File::open(&opts.config)?.read_to_string(&mut config_file)?;

    let toml_config = toml::from_str(&config_file)?;
    let dir_location = shape::get_lang_location(&toml_config, &opts.language)?;
    let output_path = &opts.output.as_path().to_str().unwrap();

    if let Some(commands) =
        shape::get_commands(&toml_config, &opts.language, shape::CommandVariants::Before)?
    {
        for command in commands {
            commands::exec(&command, &opts.project_name, &output_path)?;
        }
    }

    directory::copy_dir_all(
        &dir_location,
        &opts.output,
        &opts.project_name,
        &output_path,
    )?;

    std::env::set_current_dir(&opts.output)?;

    if let Some(commands) =
        shape::get_commands(&toml_config, &opts.language, shape::CommandVariants::After)?
    {
        for command in commands {
            commands::exec(&command, &opts.project_name, &output_path)?;
        }
    }

    Ok(())
}
