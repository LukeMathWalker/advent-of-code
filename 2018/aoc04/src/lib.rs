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

pub use crate::input::{parse_line, RawActionEntry};
pub use crate::slice_utils::LeftInclusiveSplit;
mod input;
mod slice_utils;
