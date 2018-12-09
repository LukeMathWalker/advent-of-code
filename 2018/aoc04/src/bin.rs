use chrono::{NaiveTime, Timelike};
use lib::{parse_line, Action, LeftInclusiveSplit, RawActionEntry, ShiftEntry, ShiftLog};
use std::fs::File;
use std::io::{BufRead, BufReader};

type RawShiftEntry = Vec<RawActionEntry>;

fn to_shift_log(entries: RawShiftEntry) -> ShiftLog {
    let (shift_start, action_entries) = entries.split_first().unwrap();
    let guard_id = shift_start.2.unwrap();
    let date = if shift_start.0.time() > NaiveTime::from_hms(0, 59, 0) {
        shift_start.0.date().succ()
    } else {
        shift_start.0.date()
    };
    let records = action_entries
        .iter()
        .map(|e| ShiftEntry {
            action: e.1.clone(),
            minute: e.0.clone().time().minute(),
        })
        .collect();
    ShiftLog {
        guard_id,
        date,
        records,
    }
}

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let mut raw_entries: Vec<RawActionEntry> = input.iter().map(|s| parse_line(s)).collect();
    raw_entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let raw_shift_entries: Vec<RawShiftEntry> = raw_entries
        .as_slice()
        .li_split(|x| x.1 == Action::BeginShift);
    let shift_registry: Vec<ShiftLog> = raw_shift_entries
        .into_iter()
        .map(|entries| to_shift_log(entries))
        .collect();

    //    println!("{:?}", shift_registry);
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}
