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
    #[clap(short, long)]
    /// Optional custom config file location.
    pub config: Option<PathBuf>,

    /// Project language name.
    pub language: String,
    /// Project name.
    pub project_name: String,

    #[clap(short, long)]
    /// Optional custom output directory location.
    pub output: Option<PathBuf>,
}

impl Opts {
    #[allow(clippy::self_named_constructors)]
    pub fn opts() -> Result<Self, OptError> {
        let mut opts: Opts = Opts::parse();

        opts.config = match &opts.config {
            Some(path) => Some(path.clone()),
            None => Some(PathBuf::from(std::env::var("HOME").unwrap()).join(".ctp.toml")),
        };

        if opts.config.as_ref().map(|p| !p.exists()).unwrap_or(true) {
            return Err(OptError::NoConfigFile);
        }

        if opts.output.is_none() {
            opts.output = Some(["./", &opts.project_name].iter().collect());
        }

        Ok(opts)
    }
}
