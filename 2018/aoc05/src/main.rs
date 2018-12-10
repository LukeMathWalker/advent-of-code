use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

fn read_input(input_fp: &str) -> std::io::Result<String> {
    let mut f = File::open(input_fp)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    input.pop(); // Remove newline
    Ok(input)
}

fn main() -> std::io::Result<()> {
    let input_fp = "input/part1.txt";
    let polymer = read_input(input_fp)?;
    let mut old_polymer = polymer;
    loop {
        let new_polymer = reduce(&old_polymer);
        if new_polymer == old_polymer {
            break;
        } else {
            old_polymer = new_polymer;
        }
    }
    println!("The reduce polymer has {:} units.", old_polymer.len());
    Ok(())
}

fn reduce(p: &str) -> String {
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
    String::from_iter(reduced)
}

fn are_compatible(r: &char, s: &char) -> bool {
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
