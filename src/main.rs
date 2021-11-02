#![allow(dead_code)]

mod opts;
mod shape;
use anyhow::Result;
use opts::Opts;

#[cfg(test)]
pub mod tests;

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::opts()?;
    println!("{}", opts.project_name);

    Ok(())
}
