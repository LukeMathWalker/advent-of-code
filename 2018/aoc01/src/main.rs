use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fs::File;
use std::iter::Iterator;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item=isize>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let frequencies = reader
        .lines()
        .map(|l| l.expect("Failed to read line."))
        .map(|l| isize::from_str(&l).expect("Failed to convert to integer."));
    Ok(frequencies)
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let frequencies = read_input(input_fp)?;
    let sum: isize = frequencies.sum();

    println!("Calibrated frequency: {:?}.", sum);
    Ok(())
}