use std::collections::HashMap;
use aoc_tools::input::input::read_lines;

fn plant_hash(
    plants: &mut HashMap<i64, i64>,
    middle_spot: &i64,
) -> i64 {
    let mut hash = 0i64;
    for place in middle_spot-2..middle_spot + 3 {
        if plants.contains_key(&place) {
            hash += 10i64.pow((place-(*middle_spot-2)) as u32);
        }
    }
    hash
}

fn run_generation(
    state: &mut HashMap<i64, i64>,
    mappings: &HashMap<i64, i64>,
    min_val: &mut i64,
    max_val: &mut i64,
) {
    let mut next_generation: HashMap<i64, i64> = HashMap::new();
    let (mut next_min, mut next_max) = (max_val.clone(), min_val.clone());
    for i in (*min_val-2)..(*max_val+3) {
        if mappings.contains_key(&plant_hash(state, &i)) {
            next_generation.insert(i, 1);
            if i < next_min {
                next_min = i;
            }
            if i > next_max {
                next_max = i;
            }
        }
    }
    // Upgrade the current generation to the next
    *state = next_generation;
    *min_val = next_min;
    *max_val = next_max;
}

fn main() {
    let input = read_lines::<String>("input.txt").unwrap();
    let init_state = input[0].strip_prefix("initial state: ").unwrap();
    let mut state: HashMap<i64, i64> = HashMap::new();
    let (mut min_val, mut max_val) = (input.len() as i64, 0i64);
    for (i, ch) in init_state.chars().enumerate() {
        if ch == '#' {
            state.insert(i as i64, 1);
            if (i as i64) < min_val {
                min_val = i as i64;
            }
            if (i as i64) > max_val {
                max_val = i as i64;
            }
        }
    }
    let mut mappings: HashMap<i64, i64> = HashMap::new();
    for i in 1..input.len() {
        let stuff: Vec<String> = input[i].split(" => ").into_iter().map(|x| x. to_string()).collect();
        if stuff[1] != "#" {
            continue;
        }
        let mut val = 0i64;
        for (p, b) in stuff[0].chars().enumerate() {
            if b == '#' {
                val += 10i64.pow(p as u32);
            }
        }
        mappings.insert(val, 1);
    }

    for _ in 0..20 {
        run_generation(&mut state, &mappings, &mut min_val, &mut max_val);
    }

    println!("part1: {}", state.iter().map(|(k, _)| k).sum::<i64>());

    // Note that the number of plants don't increase indefinitely and that there is a period.
    // Find periodicity and calculate forward using the relative values instead.
    // Since the order of the plants will be the same one period away, the final sum
    // can be calculated by taking the part_sum from a point X which is N * period
    // steps away from the target, and then add
    // < N * (delta x over a preiod) * number of plants >
    // to the part_sum
    let target = 50000000000i64;
    let mut hist: Vec<Vec<i64>> = Vec::new();
    let mut period: Option<i64> = None;
    let mut delta_x = 0i64;
    for i in 20..target {
        run_generation(&mut state, &mappings, &mut min_val, &mut max_val);
        if let Some(p) = period {
            if (target-i) % p == 0 {
                let part_sum = state.iter().map(|(k, _)| k).sum::<i64>();
                let periods_left = (target-(i+1)) / p;
                let final_sum = part_sum + periods_left * delta_x *state.len() as i64;
                println!("part2: {}", final_sum);
                break
            }
        }
        if i < 1000 {
            continue;
        }
        let curr_nbr = state.len() as i64;
        let curr_delta = max_val-min_val;
        for j in 0..hist.len() {
            if hist[j][0] == curr_nbr && (hist[j][2]-hist[j][1]) == (curr_delta) {
                period = Some((hist.len() - j) as i64);
                delta_x = min_val - hist[j][1];
                break
            }
        }
        hist.push(vec![curr_nbr, min_val.clone(), max_val.clone(), state.iter().map(|(k, _)| k).sum::<i64>()]);
    }
}
