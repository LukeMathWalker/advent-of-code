use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::string::String;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
    //    part2(&input_fp)?;
    Ok(())
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let box_ids = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(box_ids)
}

fn char_counter(s: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for c in s.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }
    counter
}

fn has_any_char_appearing_k_times(k: usize, c: &HashMap<char, usize>) -> bool {
    for count in c.values() {
        if count == &k {
            return true;
        }
    }
    false
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let box_ids = read_input(input_fp)?;
    let char_counters = box_ids.map(|id| char_counter(&id));
    let checks: (usize, usize) = char_counters.map(|counter| {
        (
            has_any_char_appearing_k_times(2, &counter) as usize,
            has_any_char_appearing_k_times(3, &counter) as usize,
        )
    }).fold((0, 0), |sums, values| (sums.0 + values.0, sums.1 + values.1));
    let checksum = checks.0 * checks.1;
    println!("Checksum: {:}", checksum);

    Ok(())
}

fn part2(input_fp: &str) -> std::io::Result<()> {
    unimplemented!()
}
