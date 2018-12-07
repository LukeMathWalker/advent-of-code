use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let parsed_input = input.iter().map(|s| parse_line(s));
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}

#[derive(Debug)]
struct PartialEvent {
    timestamp: NaiveDateTime,
    action: Action,
    guard_id: Option<usize>,
}

#[derive(Debug)]
struct Event {
    timestamp: NaiveDateTime,
    action: Action,
    guard_id: usize,
}

#[derive(Debug)]
enum Action {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

fn parse_line(s: &str) -> PartialEvent {
    lazy_static! {
        static ref RE_input: Regex = Regex::new(r"\[(?P<time>.{16})\]\s(?P<action>.*)").unwrap();
    }
    let captures = RE_input.captures(s).expect("Failed to parse input string.");

    let time = captures
        .name("time")
        .expect("Failed to extract datetime")
        .as_str();
    let time = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M")
        .expect("Failed to convert string representation to datetime format.");

    let action = captures
        .name("action")
        .expect("Failed to extract action")
        .as_str();
    let (action, guard_id) = parse_action(action);
    PartialEvent {
        timestamp: time,
        action,
        guard_id,
    }
}

fn parse_action(s: &str) -> (Action, Option<usize>) {
    lazy_static! {
        static ref RE_action: Regex = Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
    }
    match s {
        "wakes up" => (Action::WakesUp, None),
        "falls asleep" => (Action::FallsAsleep, None),
        s => {
            let captures = RE_action.captures(s).expect("Failed to parse action.");
            let guard_id = captures
                .name("id")
                .expect("Failed to extract guard id.")
                .as_str();
            (Action::BeginShift, Some(usize::from_str(guard_id).unwrap()))
        }
    }
}
