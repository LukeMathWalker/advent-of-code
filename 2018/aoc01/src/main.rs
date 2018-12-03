use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fs::File;
use std::iter::Iterator;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
    part2(&input_fp)?;
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item=isize>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let frequency_changes = reader
        .lines()
        .map(|l| l.expect("Failed to read line."))
        .map(|l| isize::from_str(&l).expect("Failed to convert to integer."));
    Ok(frequency_changes)
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let frequency_changes = read_input(input_fp)?;
    let frequency: isize = frequency_changes.sum();

    println!("Calibrated frequency: {:?}.", frequency);
    Ok(())
}

fn part2(input_fp: &str) -> std::io::Result<()> {
    let frequency_changes: Vec<isize> = read_input(input_fp)?.collect();
    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;

    frequencies.insert(current_frequency);
    for frequency_change in frequency_changes.iter().cycle() {
        current_frequency += frequency_change;
        let is_new_frequency = frequencies.insert(current_frequency);
        if !is_new_frequency {
            println!("First repeated frequency: {:}", current_frequency);
            break;
        }
    }
    Ok(())
}