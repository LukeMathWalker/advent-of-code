use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("../input/part01.txt")?;
    let reader = BufReader::new(f);
    let frequencies = reader
        .lines()
        .map(|l| l.expect("Failed to read line."))
        .map(|l| isize::from_str(&l).expect("Failed to convert to integer."));

    Ok(())
}