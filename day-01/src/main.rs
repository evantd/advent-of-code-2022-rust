use clap::Parser;
use color_eyre::eyre::Context;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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

    /// sum the top this-many elves' calories
    #[arg(short, long, default_value_t = 1)]
    top: usize,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let lines = read_lines(args.path)?;
    let mut heap = BinaryHeap::new();
    let mut sum: u64 = 0;

    for line in lines {
        let line = line.wrap_err("reading line")?;
        if line.is_empty() {
            // We have a blank line, so update our max and reset sum for the next elf.
            heap.push(Reverse(sum));
            if heap.len() > args.top {
                heap.pop();
            }
            sum = 0;
            //println!("{heap:?}")
        } else {
            // We have a line with a number on it, so add it to the elf's running total.
            sum = sum
                + line
                    .parse::<u64>()
                    .wrap_err(format!("while parsing {line} as u64"))?
        }
    }
    heap.push(Reverse(sum));
    if heap.len() > args.top {
        heap.pop();
    }

    println!("{}", heap.into_iter().map(|Reverse(v)| v).sum::<u64>());

    Ok(())
}

fn read_lines(path: std::path::PathBuf) -> color_eyre::Result<Lines<BufReader<File>>> {
    let file = std::fs::File::open(&path).wrap_err(format!("while opening {}", path.display()))?;
    let lines = std::io::BufReader::new(file).lines();
    Ok(lines)
}
