use chrono::NaiveDate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Clone)]
pub struct ShiftLog {
    pub guard_id: usize,
    pub date: NaiveDate,
    pub sleep: [bool; 60],
}

#[derive(Debug, Clone)]
pub struct ShiftEntry {
    pub minute: u32,
    pub action: Action,
}

impl ShiftLog {
    pub fn new(guard_id: usize, date: NaiveDate, records: Vec<ShiftEntry>) -> Self {
        ShiftLog {
            guard_id,
            date,
            sleep: ShiftLog::sleep_from_action_records(records),
        }
    }

    fn sleep_from_action_records(action_records: Vec<ShiftEntry>) -> [bool; 60]{
        let mut sleep_records = [false; 60];
        for i in 0..action_records.len() {
            let current_action_min = action_records[i].minute as usize;
            let current_state = action_records[i].action == Action::FallsAsleep;
            let next_action_min = if i == action_records.len() - 1 {
                60
            } else {
                action_records[i+1].minute as usize
            };
            for min in current_action_min..next_action_min {
                sleep_records[min] = current_state;
            }
        }
        sleep_records
    }

    pub fn sleep(&self) -> usize {
        self.sleep.iter().map(|s| if *s { 1 } else { 0 }).sum()
    }
}

pub use crate::parsing::parse_input;
mod parsing;
mod slice_utils;
