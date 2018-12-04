extern crate lib;
use lib::{BoxId, Checker};
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

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = BoxId>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let box_ids = reader
        .lines()
        .map(|l| l.expect("Failed to read line."))
        .map(|s| BoxId::from(s));
    Ok(box_ids)
}

fn part1(input_fp: &str) -> std::io::Result<()> {
    let box_ids: Vec<BoxId> = read_input(input_fp)?.collect();
    let checksum = box_ids.as_slice().check_sum();
    println!("Checksum: {:}", checksum);

    Ok(())
}

fn find_boxes_with_suit(box_ids: &Vec<String>) -> Option<(&str, &str)> {
    let n = box_ids.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if are_close(&box_ids[i], &box_ids[j]) {
                return Some((&box_ids[i], &box_ids[j]));
            }
        }
    }
    None
}

fn are_close(first_box_id: &str, second_box_id: &str) -> bool {
    let mut n_mismatches = 0;
    for (c, d) in first_box_id.chars().zip(second_box_id.chars()) {
        if c != d {
            n_mismatches += 1;
            if n_mismatches > 1 {
                return false;
            }
        }
    }
    true
}

fn common_chars(s: &str, t: &str) -> Vec<char> {
    let mut common = vec![];
    for (c, d) in s.chars().zip(t.chars()) {
        if c == d {
            common.push(c);
        }
    }
    common
}

//fn part2(input_fp: &str) -> std::io::Result<()> {
//    let box_ids: Vec<String> = read_input(input_fp)?.collect();
//    let box_ids_with_suit = find_boxes_with_suit(&box_ids).expect("No ids are close.");
//    println!("Box ids with suit: {:?}", box_ids_with_suit);
//    let common_letters: String = common_chars(box_ids_with_suit.0, box_ids_with_suit.1)
//        .into_iter()
//        .collect();
//    println!("Letter in common: {:?}", common_letters);
//    Ok(())
//}
