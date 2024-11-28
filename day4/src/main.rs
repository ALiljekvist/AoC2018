use aoc_tools::input::*;
use input::read_lines;
use std::collections::HashMap;

fn main() {
    let mut entries = read_lines::<String>("input.txt").unwrap();
    entries.sort();
    let mut guards: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut curr_id = 0u64;
    let mut curr_mins: Vec<u64> = vec![0; 60];
    let mut fell_asleep = 0usize;
    for entry in entries {
        let ind = entry.find("]").unwrap();
        let info = entry[ind+2..].to_string();
        let min = entry[ind-2..ind].parse::<usize>().unwrap();
        match info.as_str() {
            "falls asleep" => {
                fell_asleep = min;
            },
            "wakes up" => {
                // Add number of minutes slept to the guards info
                for i in fell_asleep..min {
                    curr_mins[i] += 1;
                }
                fell_asleep = 0;
            },
            _ => {
                // Assume that the message is the start of a shift
                guards.insert(curr_id, curr_mins);
                curr_id = info.chars()
                .into_iter()
                .filter(|x| x.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
                curr_mins = match guards.get(&curr_id) {
                    Some(mins) => mins.to_vec(),
                    None => vec![0; 60]
                };
            }
        }
    }
    guards.insert(curr_id, curr_mins);

    let mut max_mins = 0u64;
    let mut max_single_minute = 0u64;
    let mut part1 = 0u64;
    let mut part2 = 0u64;
    for (id, mins) in guards.iter() {
        let mut minute_chosen = 0usize;
        let mut tot_mins = 0u64;
        for i in 0..mins.len() {
            tot_mins += mins[i];
            if mins[i] > mins[minute_chosen] {
                if mins[i] > max_single_minute {
                    part2 = id * i as u64;
                    max_single_minute = mins[i];
                }
                minute_chosen = i;
            }
        }
        if tot_mins > max_mins {
            part1 = id * minute_chosen as u64;
            max_mins = tot_mins
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
