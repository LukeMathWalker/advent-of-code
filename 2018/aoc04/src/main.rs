use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let input: Vec<String> = read_input(input_fp)?.collect();
    let partial_events: Vec<PartialEvent> = input.iter().map(|s| parse_line(s)).collect();

    let mut raw_shifts_calendar = RawShiftsCalendar::new();
    for partial_event in partial_events.iter() {
        let events = raw_shifts_calendar.entry(partial_event.timestamp.date()).or_insert(vec![]);
        (*events).push(partial_event);
    }


    let mut shifts_calendar = ShiftsCalendar::new();
    for (date, events) in raw_shifts_calendar.iter() {
        let mut records: Vec<ShiftEntry> = events.iter().map(
            |e| ShiftEntry { time: e.timestamp.time(), action: e.action.clone() }
        ).collect();
        records.sort_by(
            |e, f| e.time.cmp(&f.time)
        );
        let mut guard_id: Option<usize> = None;
        for event in events.iter() {
            println!("{:?}", event);
            match event.guard_id {
                Some(id) => {
                    guard_id.replace(id);
                    break;
                }
                _ => (),
            }
        }
        shifts_calendar.insert(*date, ShiftLog{ guard_id: guard_id.unwrap(), records });
    }
    println!("Log: {:?}.", shifts_calendar);
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}

type RawShiftsCalendar<'a> = HashMap<NaiveDate, Vec<&'a PartialEvent>>;

type ShiftsCalendar<'a> = HashMap<NaiveDate, ShiftLog>;

#[derive(Debug, Clone)]
struct ShiftLog {
    pub guard_id: usize,
    pub records: Vec<ShiftEntry>,
}

#[derive(Debug, Clone)]
struct ShiftEntry {
    pub time: NaiveTime,
    pub action: Action,
}

#[derive(Debug, Clone)]
struct PartialEvent {
    pub timestamp: NaiveDateTime,
    pub action: Action,
    pub guard_id: Option<usize>,
}

#[derive(Debug, Clone)]
struct Event {
    pub timestamp: NaiveDateTime,
    pub action: Action,
    pub guard_id: usize,
}

#[derive(Debug, Clone)]
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
