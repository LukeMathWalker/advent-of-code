use chrono::NaiveDate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Clone)]
pub struct ShiftLog {
    pub guard_id: usize,
    pub date: NaiveDate,
    pub records: Vec<ShiftEntry>,
}

#[derive(Debug, Clone)]
pub struct ShiftEntry {
    pub minute: u32,
    pub action: Action,
}

impl ShiftLog {
    pub fn sleep(&self) -> usize {
        let mut minutes = 0;
        let mut iter = self.records.iter().peekable();
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
}

pub use crate::parsing::parse_input;
mod parsing;
mod slice_utils;
