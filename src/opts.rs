use std::path::Path;

use anyhow::Result;
use clap::Parser;
use thiserror::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_PROJECT_LOCATION: &str = "_";

#[derive(Error, Debug)]
pub enum OptError {
    #[error("No config file found. Please create one at $HOME/.ctp or pass in a config file location with --config.")]
    NoConfigFile,

    #[error("{0} is not a valid output directory name, please use --output for a valid directory name or use a differnet project name.")]
    ProjectNameError(String),

    #[error("\"{0}\" is not a valid output directory name, please enter a new output directory.")]
    OutputError(String),
}

#[derive(Parser)]
#[clap(version = VERSION, author = "William Lane <williamlane923@gmail.com>")]
pub struct Opts {
    #[clap(short, long, default_value = "~/.ctp")]
    /// Optional custom config file location.
    pub config: String,

    /// Project language name.
    pub language: String,
    /// Project name.
    pub project_name: String,

    #[clap(short, long, default_value = DEFAULT_PROJECT_LOCATION)]
    /// Optional custom output directory location.
    pub output: String,
}

impl Opts {
    pub fn valid_name(s: &str) -> bool {
        [
            '#', '\\', '%', '&', '{', '}', '<', '>', '*', '?', '/', '$', '!', '\'', '"', ':', '+',
            '`', '|', '=',
        ]
        .iter()
        .any(|c| s.contains(*c))
    }

    pub fn config_file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    #[allow(clippy::self_named_constructors)]
    pub fn opts() -> Result<Self, OptError> {
        let mut opts: Opts = Opts::parse();

        if opts.output == DEFAULT_PROJECT_LOCATION {
            opts.output = opts.project_name.to_owned();
        }

        let output_invalid = Self::valid_name(&opts.output);
        let project_name_invalid = Self::valid_name(&opts.project_name);

        if !Self::config_file_exists(&opts.config) {
            return Err(OptError::NoConfigFile);
        }

        if output_invalid {
            return Err(OptError::OutputError(opts.output));
        }

        if project_name_invalid && output_invalid {
            return Err(OptError::ProjectNameError(opts.project_name));
        }

        Ok(opts)
    }
}
