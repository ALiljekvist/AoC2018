use aoc_tools::input::*;
use input::read_lines;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Worker {
    time_left: i64,
    step: String
}

fn reduce(steps: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    let mut new_possible_steps: Vec<String> = Vec::new();
    for (steps, preds) in steps.iter() {
        if preds.len() == 0 {
            new_possible_steps.push(steps.clone());
        }
    }
    for step in new_possible_steps.iter() {
        steps.remove(step);
    }
    new_possible_steps
}

fn perform_step(steps: &mut HashMap<String, Vec<String>>, step: &String) -> Vec<String> {
    for (_, preds) in steps.iter_mut() {
        if let Some(ind) = preds.iter().position(|x: &String| x == step) {
            // Remove step from each preds-vector
            preds.remove(ind);
        }
    }
    reduce(steps)
} 

fn calc_time(s: &String) -> i64 {
    if s == "" {
        return 0;
    }
    let extra = " ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let mut time = 0i64;
    if let Some(ind) = extra.find(s) {
        time = 60 + ind as i64;
    }
    time
}

fn main() {
    let rules: Vec<(String,String)> = read_lines::<String>("input.txt")
        .unwrap()
        .into_iter()
        .map(|s| {
            let parts: Vec<String> = s.split(" ").map(|x| x.parse().unwrap()).collect();
            (parts[1].clone(), parts[7].clone())
        })
        .collect();
    let mut steps: HashMap<String,Vec<String>> = HashMap::new();
    for (step1, step2) in rules {
        let step = steps.entry(step2).or_default();
        step.push(step1.clone());
    
        if !steps.contains_key(&step1) {
            // Other step has not been seen yet, add empty vector
            steps.insert(step1, Vec::new());
        }
    }
    let mut steps1 = steps.clone();
    let mut next = reduce(&mut steps1);
    let mut done: Vec<String> = Vec::new();
    while next.len() > 0 {
        next.sort();
        let step = next.remove(0);
        let available = perform_step(&mut steps1, &step);
        for new_step in available {
            next.push(new_step);
        }
        done.push(step);
    }
    println!("part1: {}", done.join("").parse::<String>().unwrap());

    let mut workers: Vec<Worker> = Vec::new();
    let mut next = reduce(&mut steps);
    next.sort();
    for _ in 0..5 {
        workers.push(Worker{time_left:0, step: "".to_string()});
    }
    let mut tot_time = 0i64;
    while next.len() > 0 || steps.len() > 0 {
        // Get the next worker to perform a step and/or give a new step
        workers.sort_unstable_by_key(|x| x.time_left);
        let worker_ind = if next.len() > 0 {
            // There are still steps to be handed out, get the
            // first worker.
            0
        } else {
            // No more more steps to hand out. Find the worker with
            // the step that finishes next.
            let mut ind = 0usize;
            while workers[ind].time_left == 0 &&
                ind < workers.len() - 1 {
                ind += 1
            }
            ind
        };
        let mut curr_worker = workers.remove(worker_ind);
        // If the current worker has a step to perform, add it to the
        // total time and adda the newly unlocked steps to the queue
        if curr_worker.step != "" {
            tot_time += curr_worker.time_left;
            for work_ind in worker_ind..workers.len() {
                workers[work_ind].time_left -= curr_worker.time_left;
            }
            let new_possible_steps = perform_step(&mut steps, &curr_worker.step);
            for new_step in new_possible_steps {
                next.push(new_step);
            }
        }
        
        let next_step = if next.len() > 0 {
            // Sort the next available steps and get the next in queue
            next.sort();
            next.remove(0)
        } else {
            // No new steps unlocked, give an empty step to this worker
            "".to_string()
        };
        curr_worker.time_left = calc_time(&next_step);
        curr_worker.step = next_step;
        workers.push(curr_worker);
        workers.sort_unstable_by_key(|x| x.time_left);
    }
    if let Some(final_worker) = workers.pop() {
        tot_time += final_worker.time_left
    }
    println!("part2: {}", tot_time);
}
