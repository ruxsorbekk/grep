use clap::Parser;
use anyhow::{Context, Result};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        pb.set_message("In progress...");
        let args = Cli::parse();
        let path = args.path;
        let content = std::fs::read_to_string(&path).with_context(|| format!("could not read file `{:?}`", path))?;
        gureppu::find_matches(&content, &args.pattern, &mut std::io::stdout());
        pb.finish_with_message("Done");
    
    Ok(())
}