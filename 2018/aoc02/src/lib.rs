use std::collections::HashMap;

pub struct BoxId {
    id: String,
}

type CharCounter = HashMap<char, usize>;

impl From<String> for BoxId {
    fn from(s: String) -> Self {
        BoxId {id: s}
    }
}

impl BoxId {
    pub fn count(&self) -> CharCounter {
        let mut counter = CharCounter::new();
        for c in self.id.chars() {
            let count = counter.entry(c).or_insert(0);
            *count += 1;
        }
        counter
    }
}

pub trait Checker {
    fn check_sum(&self) -> usize;
}

impl<'a> Checker for &'a [BoxId] {
    fn check_sum(&self) -> usize {
        let checks: (usize, usize) = self
            .iter()
            .map(|id| id.count())
            .map(|counter| {
                (
                    has_any_char_appearing_k_times(2, &counter) as usize,
                    has_any_char_appearing_k_times(3, &counter) as usize,
                )
            }).fold((0, 0), |sums, values| {
            (sums.0 + values.0, sums.1 + values.1)
        });
        let checksum = checks.0 * checks.1;
        checksum
    }
}

fn has_any_char_appearing_k_times(k: usize, c: &CharCounter) -> bool {
    for count in c.values() {
        if count == &k {
            return true;
        }
    }
    false
}

