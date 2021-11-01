mod opts;
use opts::Opts;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::opts()?;
    println!("{}", opts.project_name);

    Ok(())
}
