use lib::{parse_input, Action, ShiftLog, ShiftEntry};
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
        *time_asleep += minutes_asleep(shift.records.as_slice());
    }
    println!("{:?}", sleep_registry);
    Ok(())
}

fn minutes_asleep(records: &[ShiftEntry]) -> usize {
    let mut minutes = 0;
    let mut iter = records.iter().peekable();
    loop {
        match iter.next() {
            Some(record) => {
                if record.action == Action::FallsAsleep {
                    let wakeup = match iter.peek() {
                        Some(entry) => entry.minute,
                        None => 59,
                    };
                    minutes += wakeup - record.minute;
                }
            },
            None => break,
        }
    };
    minutes as usize
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}
