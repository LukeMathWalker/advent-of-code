use std::io::{BufReader, BufRead};
use std::string::String;
use std::fs::File;
use std::iter::Iterator;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
//    part2(&input_fp)?;
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item=String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let box_ids = reader
        .lines()
        .map(|l| l.expect("Failed to read line."));
    Ok(box_ids)
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let box_ids = read_input(input_fp)?;

    Ok(())
}

fn part2(input_fp: &str) -> std::io::Result<()> {
    unimplemented!()
}

