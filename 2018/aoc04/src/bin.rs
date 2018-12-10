use lib::{parse_input, EmployeeSleepLog, ShiftLog};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let shift_registry: Vec<ShiftLog> = parse_input(input);

    let mut shifts_by_employee = HashMap::new();
    for shift in shift_registry.iter() {
        let shifts = shifts_by_employee.entry(shift.guard_id).or_insert(vec![]);
        shifts.push(shift);
    }

    let mut sleep_registry = HashMap::new();
    for (guard_id, shifts) in shifts_by_employee.iter() {
        let sleep_log = EmployeeSleepLog::new(shifts.as_slice());
        sleep_registry.insert(guard_id, sleep_log);
    }

    let (sleepiest_guard_id, sleepiest_guard_log) = sleep_registry
        .iter()
        .max_by_key(|t| t.1.sleep_time())
        .unwrap();
    let sleepiest_minute = sleepiest_guard_log.sleepiest_minute().0;

    println!(
        "Sleepiest guard id: {:?}. Sleepiest minute: {:?}",
        sleepiest_guard_id, sleepiest_minute
    );
    println!(
        "Part 1 solution: {:?}",
        *sleepiest_guard_id * sleepiest_minute
    );

    let (sleepiest_guard_id, sleepiest_guard_log) = sleep_registry
        .iter()
        .max_by_key(|t| t.1.sleepiest_minute().1)
        .unwrap();
    let sleepiest_minute = sleepiest_guard_log.sleepiest_minute().0;
    println!(
        "Part 2 solution: {:?}",
        *sleepiest_guard_id *sleepiest_minute
    );
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}
