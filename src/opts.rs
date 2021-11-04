use std::path::PathBuf;

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
    #[clap(short, long, default_value = "_default_")]
    /// Optional custom config file location.
    pub config: PathBuf,

    /// Project language name.
    pub language: String,
    /// Project name.
    pub project_name: String,

    #[clap(short, long, default_value = DEFAULT_PROJECT_LOCATION)]
    /// Optional custom output directory location.
    pub output: PathBuf,
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

    #[allow(clippy::self_named_constructors)]
    pub fn opts() -> Result<Self, OptError> {
        let mut opts: Opts = Opts::parse();

        if opts.config.as_path().to_str().unwrap() == "_default_" {
            opts.config = PathBuf::from(std::env::var("HOME").unwrap()).join(".ctp.toml");
        }

        if opts.output.as_path().to_str().unwrap() == DEFAULT_PROJECT_LOCATION {
            opts.output = ["./", &opts.project_name].iter().collect();
        }

        println!("{}", opts.config.canonicalize().unwrap().to_str().unwrap());
        if !opts.config.exists() {
            println!("{}", opts.config.as_path().to_str().unwrap());
            return Err(OptError::NoConfigFile);
        }

        if Self::valid_name(&opts.project_name) {
            return Err(OptError::ProjectNameError(opts.project_name));
        }

        Ok(opts)
    }
}
