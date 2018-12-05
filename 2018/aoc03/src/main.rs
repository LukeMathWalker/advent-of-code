#![feature(try_trait)]
extern crate lazy_static;
extern crate regex;
extern crate ndarray;
use ndarray::{Array, Array2, s};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let side_length = 1000;
    let fabric_claims = read_fabric_claims(input_fp, &side_length)?;
    let fabric_w_claims = part1(fabric_claims.as_slice(), side_length);
    part2(fabric_claims.as_slice(), &fabric_w_claims);

    Ok(())
}

#[derive(Debug)]
pub struct Claim {
    pub id: usize,
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl Claim {
    pub fn from_str(s: &str, fabric_side_length: &usize) -> Result<Self, std::option::NoneError> {
        let caps = Claim::parse(s).unwrap();
        let id = usize::from_str(caps.get(1)?.as_str()).unwrap();
        let min_x = usize::from_str(caps.get(2)?.as_str()).unwrap();
        let max_y = fabric_side_length - usize::from_str(caps.get(3)?.as_str()).unwrap();
        let max_x = min_x + usize::from_str(caps.get(4)?.as_str()).unwrap();
        let min_y = max_y - usize::from_str(caps.get(5)?.as_str()).unwrap();
        Ok(Claim {
            id,
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    fn parse(s: &str) -> Option<Captures> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"#(\d{1,4}) @ (\d{1,4}),(\d{1,4}): (\d{1,4})x(\d{1,4})").unwrap();
        }
        RE.captures(s)
    }
}

fn read_fabric_claims(input_fp: &str, fabric_side_length: &usize) -> std::io::Result<Vec<Claim>> {
    let lines = read_input(input_fp)?;
    let rectangles: Vec<Claim> = lines
        .map(|l| Claim::from_str(&l, fabric_side_length).unwrap())
        .collect();
    Ok(rectangles)
}

fn read_input(input_fp: &str) -> std::io::Result<impl Iterator<Item = String>> {
    let f = File::open(input_fp)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.expect("Failed to read line."));
    Ok(lines)
}

fn part1(fabric_claims: &[Claim], fabric_side_length: usize) -> Array2<usize> {
    let mut fabric = Array::zeros((fabric_side_length.clone(), fabric_side_length.clone()));
    for fc in fabric_claims {
        let mut fabric_patch = fabric.slice_mut(s![fc.min_x..fc.max_x, fc.min_y..fc.max_y]);
        fabric_patch += 1_usize;
    }
    let n_spots = fabric.mapv(|x| {if x > 1 { 1 } else { 0 }}).sum();
    println!("Number of spots claimed at least twice: {:}", n_spots);
    fabric
}

fn part2(fabric_claims: &[Claim], fabric_w_claims: &Array2<usize>) {
    for fc in fabric_claims {
        let fabric_patch = fabric_w_claims.slice(s![fc.min_x..fc.max_x, fc.min_y..fc.max_y]);
        let n_spots = fabric_patch.map(|x| {if x > &1 { 1 } else { 0 }}).sum();
        if n_spots == 0 {
            println!("Found it! Id: {:?}", fc.id);
            break;
        }
    }
}
