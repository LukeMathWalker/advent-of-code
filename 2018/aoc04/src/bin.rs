use lib::{parse_input, ShiftLog};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let shift_registry: Vec<ShiftLog> = parse_input(input);

    let mut sleep_registry = HashMap::new();
    for shift in shift_registry.iter() {
        let time_asleep = sleep_registry.entry(shift.guard_id).or_insert(0);
        *time_asleep += shift.sleep();
    }

    println!("{:?}", sleep_registry);
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}
