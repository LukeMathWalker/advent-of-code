use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use lib::{Action, RawActionEntry, LeftInclusiveSplit, parse_line};

type RawShiftEntry = Vec<RawActionEntry>;

fn to_shift_log(entries: RawShiftEntry) -> ShiftLog {
    let first_shift_entry = entries.first().unwrap();
    let guard_id = first_shift_entry.2.unwrap();
    let records = entries.into_iter().map(|e| ShiftEntry { action: e.1, time: e.0 }).collect();
    ShiftLog { guard_id, records }
}

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let mut raw_entries: Vec<RawActionEntry> = input.iter().map(|s| parse_line(s)).collect();
    raw_entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut raw_shift_entries: Vec<RawShiftEntry> = raw_entries.
        as_slice().
        li_split(|x| x.1 == Action::BeginShift);
    let shift_registry: Vec<ShiftLog> = raw_shift_entries.into_iter().map(|entries| to_shift_log(entries)).collect();

    println!("{:?}", shift_registry);
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}

#[derive(Debug, Clone)]
struct ShiftLog {
    pub guard_id: usize,
    pub records: Vec<ShiftEntry>,
}

#[derive(Debug, Clone)]
struct ShiftEntry {
    pub time: NaiveDateTime,
    pub action: Action,
}
