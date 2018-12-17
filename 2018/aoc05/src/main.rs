use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

type Molecule = char;
type Polymer = String;

fn read_input(input_fp: &str) -> std::io::Result<Polymer> {
    let mut f = File::open(input_fp)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    input.pop(); // Remove newline
    Ok(input)
}

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let polymer = read_input(input_fp)?;
    let reduced_polymer = reduce(&polymer);
    println!("The reduced polymer has {:} units.", reduced_polymer.len());
    let molecules: HashSet<Molecule> = polymer.to_ascii_lowercase().chars().collect();
    let mut removal_gains: HashMap<Molecule, usize> = HashMap::new();
    for molecule in molecules {
        let l: Polymer = polymer
            .chars()
            .filter(|m| m.to_ascii_lowercase() != molecule)
            .collect();
        removal_gains.insert(molecule, reduce(&l).len());
    }
    println!(
        "The shortest length we can achieve is {:?}.",
        removal_gains.values().min().unwrap()
    );
    Ok(())
}

fn reduce(p: &Polymer) -> Polymer {
    let mut old = p.to_string();
    loop {
        let new = _reduce(&old);
        if new == old {
            break;
        } else {
            old = new;
        }
    }
    old
}

fn _reduce(p: &Polymer) -> Polymer {
    let mut iter = p.chars().peekable();
    let mut reduced = vec![];
    loop {
        match iter.next() {
            None => {
                break;
            }
            Some(reactant) => {
                let next_reactant = iter.peek();
                match next_reactant {
                    None => {
                        reduced.push(reactant);
                        break;
                    }
                    Some(next_reactant) => {
                        if are_compatible(&reactant, &next_reactant) {
                            let _ = iter.next();
                        } else {
                            reduced.push(reactant);
                        }
                    }
                }
            }
        }
    }
    Polymer::from_iter(reduced)
}

fn are_compatible(r: &Molecule, s: &Molecule) -> bool {
    if r.to_ascii_lowercase() == s.to_ascii_lowercase() {
        if r.is_ascii_lowercase() == s.is_ascii_uppercase() {
            true
        } else {
            false
        }
    } else {
        false
    }
}
