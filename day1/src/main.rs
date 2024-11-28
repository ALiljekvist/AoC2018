use std::fs;
use std::collections::HashSet;

fn main() {
    let input: Vec<i64> = fs::read_to_string("input.txt")
    .unwrap()
    .split('\n')
    .filter(|x| !x.is_empty())
    .map(|s| s.parse().unwrap())
    .collect();

    let mut p1: i64 = input.iter().sum();
    println!("part1: {}", p1);

    p1 = 0;
    let mut p2: Option<i64> = None;
    let mut freq_set: HashSet<i64> = HashSet::new();
    while p2 == None {
        for i in input.iter() {
            p1 += i;
            if let Some(_) = p2 {
                continue;
            }
            if freq_set.contains(&p1) {
                p2 = Some(p1);
                continue;
            }
            freq_set.insert(p1.clone());
        };
    }
    if let Some(val) = p2 {
        println!("part2: {}", val);
    }
}
