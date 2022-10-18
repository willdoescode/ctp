use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptError {
    #[error("No config file found. Please create one at $HOME/.ctp.toml or pass in a config file location with --config.")]
    NoConfigFile,
}

#[derive(Parser)]
#[clap(version = crate::VERSION, author = "William Lane <williamlane923@gmail.com>")]
pub struct Opts {
    #[clap(short, long, default_value = "_default_")]
    /// Optional custom config file location.
    pub config: PathBuf,

    /// Project language name.
    pub language: String,
    /// Project name.
    pub project_name: String,

    #[clap(short, long, default_value = crate::DEFAULT_PROJECT_LOCATION)]
    /// Optional custom output directory location.
    pub output: PathBuf,
}

impl Opts {
    #[allow(clippy::self_named_constructors)]
    pub fn opts() -> Result<Self, OptError> {
        let mut opts: Opts = Opts::parse();

        if opts.config.as_path().to_str().unwrap() == "_default_" {
            opts.config = PathBuf::from(std::env::var("HOME").unwrap()).join(".ctp.toml");
        }

        if opts.output.as_path().to_str().unwrap() == crate::DEFAULT_PROJECT_LOCATION {
            opts.output = ["./", &opts.project_name].iter().collect();
        }

        if !opts.config.exists() {
            return Err(OptError::NoConfigFile);
        }

        Ok(opts)
    }
}
