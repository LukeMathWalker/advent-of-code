use super::{Action, ShiftLog, ShiftEntry};
use crate::slice_utils::LeftInclusiveSplit;
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

type RawActionEntry = (NaiveDateTime, Action, Option<usize>);

pub fn parse_input(input: Vec<String>) -> Vec<ShiftLog> {
    let mut raw_entries: Vec<RawActionEntry> = input.iter().map(|s| parse_line(s)).collect();
    raw_entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let raw_shift_entries: Vec<RawShiftEntry> = raw_entries
        .as_slice()
        .li_split(|x| x.1 == Action::BeginShift);
    let shift_registry: Vec<ShiftLog> = raw_shift_entries
        .into_iter()
        .map(|entries| to_shift_log(entries))
        .collect();
    shift_registry
}

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
    ShiftLog::new(
        guard_id,
        date,
        records,
    )
}

fn parse_line(s: &str) -> RawActionEntry {
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
    (time, action, guard_id)
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
