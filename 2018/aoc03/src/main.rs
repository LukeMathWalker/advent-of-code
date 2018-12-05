#![feature(try_trait)]
extern crate lazy_static;
extern crate regex;
extern crate ndarray;
use ndarray::{Array, s};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let side_length = 1000;
    part1(&input_fp, side_length)?;
    Ok(())
}

#[derive(Debug)]
pub struct Rectangle {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl Rectangle {
    pub fn from_str(s: &str, fabric_side_length: usize) -> Result<Self, std::option::NoneError> {
        let caps = Rectangle::parse(s).unwrap();
        let min_x = usize::from_str(caps.get(1)?.as_str()).unwrap();
        let max_y = fabric_side_length - usize::from_str(caps.get(2)?.as_str()).unwrap();
        let max_x = min_x + usize::from_str(caps.get(3)?.as_str()).unwrap();
        let min_y = max_y - usize::from_str(caps.get(4)?.as_str()).unwrap();
        Ok(Rectangle {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    fn parse(s: &str) -> Option<Captures> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"#\d{1,4} @ (\d{1,4}),(\d{1,4}): (\d{1,4})x(\d{1,4})").unwrap();
        }
        RE.captures(s)
    }
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}

fn part1(input_fp: &str, fabric_side_length: usize) -> std::io::Result<()> {
    let lines = read_input(input_fp)?;
    let rectangles: Vec<Rectangle> = lines
        .map(|l| Rectangle::from_str(&l, fabric_side_length).unwrap())
        .collect();
    let mut fabric = Array::zeros((fabric_side_length.clone(), fabric_side_length.clone()));
    for r in rectangles {
        let mut fabric_patch = fabric.slice_mut(s![r.min_x..r.max_x, r.min_y..r.max_y]);
        fabric_patch += 1_usize;
    }
    let mut counter = 0;
    for spot in fabric.iter() {
        if spot > &1 {
            counter += 1;
        }

    }
    println!("Number of spots claimed at least twice: {:}", counter);
    Ok(())
}
