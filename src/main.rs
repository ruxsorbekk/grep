use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    
    let mut result: Vec<String> = Vec::new();
    // let result = reader.lines().into_iter().filter(|x| x.as_ref().unwrap().contains(&args.pattern));
    // println!("{:?}", result);
    for line in reader.lines().into_iter() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            result.push(line.unwrap());
        }
    }
    println!("{:?}", result);
    Ok(())
}
