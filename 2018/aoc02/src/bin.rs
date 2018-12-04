extern crate lib;
use lib::{BoxId, Checker};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::string::String;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    part1(&input_fp)?;
    part2(&input_fp)?;
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

fn find_boxes_with_suit(box_ids: &Vec<BoxId>) -> Option<(&BoxId, &BoxId)> {
    let n = box_ids.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if (&box_ids[i]).is_close_to(&box_ids[j]) {
                return Some((&box_ids[i], &box_ids[j]));
            }
        }
    }
    None
}

fn part2(input_fp: &str) -> std::io::Result<()> {
    let box_ids: Vec<BoxId> = read_input(input_fp)?.collect();
    let box_ids_with_suit = find_boxes_with_suit(&box_ids).expect("No box ids are close.");
    println!("Box ids containing the suit: {:?}", box_ids_with_suit);
    let common_letters: String = box_ids_with_suit.0.intersect(box_ids_with_suit.1)
        .into_iter()
        .collect();
    println!("Common letters: {:?}", common_letters);
    Ok(())
}
