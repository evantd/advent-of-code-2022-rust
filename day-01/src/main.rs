use clap::Parser;
use color_eyre::eyre::Context;

/// Solver for Advent of Code Day 1 (2022, Rust)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// path to a file containing calorie counts for each Elf's food, with one food per line and blank lines separating elves
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let input = read_input(args.path)?;
    println!("{input}");

    Ok(())
}

fn read_input(path: std::path::PathBuf) -> color_eyre::Result<String> {
    let input = std::fs::read_to_string(&path).wrap_err(format!("reading {}", path.display()))?;
    Ok(input)
}
