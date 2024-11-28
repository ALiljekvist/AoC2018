use aoc_tools::input::*;
use input::read_lines;
use std::collections::HashSet;

fn reduce(polymer: String) -> (String, bool) {
    let units: Vec<char> = polymer.chars().into_iter().collect();

    let mut reduced = false;
    let mut new_polymer: Vec<char> = Vec::new();
    let mut i = 0usize;
    while i < units.len() {
        if i + 1 == units.len() {
            new_polymer.push(units[i]);
            break
        }
        if units[i] != units[i+1] &&
            units[i].to_ascii_lowercase() == units[i+1].to_ascii_lowercase() {
                reduced = true;
                i += 2;
                continue
        }
        new_polymer.push(units[i]);
        i += 1;
    }

    return (new_polymer.into_iter().collect(), reduced);
}

fn main() {
    let mut polymer = read_lines::<String>("input.txt")
        .unwrap()
        .pop()
        .unwrap();
    let mut reduced = true;
    while reduced {
        (polymer, reduced) = reduce(polymer);
    }
    println!("{}", polymer.len());

    let units_to_remove: HashSet<char> = polymer.clone()
        .chars()
        .into_iter()
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let mut min_chain_len = polymer.len();
    for unit in units_to_remove {
        let mut reduced_polymer = polymer.clone()
            .replace(unit, "")
            .replace(unit.to_ascii_uppercase(), "");
        let mut reduced = true;
        while reduced {
            (reduced_polymer, reduced) = reduce(reduced_polymer);
        }
        if reduced_polymer.len() < min_chain_len {
            min_chain_len = reduced_polymer.len()
        }
    }
    println!("{}", min_chain_len);
}
