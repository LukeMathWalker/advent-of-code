use chrono::NaiveDate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum State {
    Asleep,
    Awake,
}

pub struct GuardSleepLog {
    pub guard_id: usize,
    pub sleep: [usize; 60],
}

impl GuardSleepLog {
    pub fn new(shift_logs: &[&ShiftLog]) -> Self {
        let guard_id = shift_logs
            .first()
            .expect("At least one log must be passed.")
            .guard_id;
        let mut sleep = [0; 60];
        for shift_log in shift_logs {
            assert_eq!(guard_id, shift_log.guard_id);
            for (min, state) in shift_log.sleep.iter().enumerate() {
                if *state == State::Asleep {
                    sleep[min] += 1;
                }
            }
        }
        GuardSleepLog { guard_id, sleep }
    }

    pub fn sleep_time(&self) -> usize {
        self.sleep.iter().sum()
    }

    pub fn sleepiest_minute(&self) -> (usize, usize) {
        let mut max_index = 0;
        let mut max_value = self.sleep[0];
        for (i, value) in self.sleep.iter().enumerate() {
            if value > &max_value {
                max_index = i;
                max_value = *value;
            }
        }
        (max_index, max_value)
    }
}

#[derive(Clone)]
pub struct ShiftLog {
    pub guard_id: usize,
    pub date: NaiveDate,
    pub sleep: [State; 60],
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

    fn sleep_from_action_records(action_records: Vec<ShiftEntry>) -> [State; 60] {
        let mut sleep_records = [State::Awake; 60];
        for i in 0..action_records.len() {
            let current_action_min = action_records[i].minute as usize;
            let current_state = if action_records[i].action == Action::FallsAsleep {
                State::Asleep
            } else {
                State::Awake
            };
            let next_action_min = if i == action_records.len() - 1 {
                60
            } else {
                action_records[i + 1].minute as usize
            };
            for min in current_action_min..next_action_min {
                sleep_records[min] = current_state;
            }
        }
        sleep_records
    }

    pub fn sleep(&self) -> usize {
        self.sleep.iter().map(|s| if *s == State::Asleep { 1 } else { 0 }).sum()
    }
}

pub use crate::parsing::parse_input;
mod parsing;
mod slice_utils;
