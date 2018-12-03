use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
    Ok(())
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let sum: isize = reader
        .lines()
        .map(|l| l.expect("Failed to read line."))
        .map(|l| isize::from_str(&l).expect("Failed to convert to integer."))
        .sum();

    println!("Calibrated frequency: {:?}.", sum);
    Ok(())
}