use super::Action;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub type RawActionEntry = (NaiveDateTime, Action, Option<usize>);

pub fn parse_line(s: &str) -> RawActionEntry {
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
