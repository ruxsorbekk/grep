use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let path = args.path;
    let content = 
        std::fs::read_to_string(&path).with_context(|| format!("could not read file `{:?}`", path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}