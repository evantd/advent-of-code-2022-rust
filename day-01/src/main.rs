use clap::Parser;
use color_eyre::eyre::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

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

    let lines = read_lines(args.path)?;
    let mut max: Option<u64> = None;
    let mut sum: u64 = 0;

    for line in lines {
        let line = line.wrap_err("reading line")?;
        match line.is_empty() {
            false => {
                // We have a line with a number on it, so add it to the elf's running total.
                sum = sum
                    + line
                        .parse::<u64>()
                        .wrap_err(format!("while parsing {line} as u64"))?
            }
            true => {
                // We have a blank line, so update our max and reset sum for the next elf.
                max = std::cmp::max(max, Some(sum));
                sum = 0
            }
        }
    }
    max = std::cmp::max(max, Some(sum));

    println!("{max:?}");

    Ok(())
}

fn read_lines(path: std::path::PathBuf) -> color_eyre::Result<Lines<BufReader<File>>> {
    let file = std::fs::File::open(&path).wrap_err(format!("while opening {}", path.display()))?;
    let lines = std::io::BufReader::new(file).lines();
    Ok(lines)
}
