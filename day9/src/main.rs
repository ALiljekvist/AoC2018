use aoc_tools::input::*;
use input::read_lines;
use std::collections::LinkedList;

fn place(marbles: &mut LinkedList<u64>, next_marble: u64) -> u64 {
    if next_marble % 23 == 0 {
        // No need to check bounds since we always reposition at the 0 index as
        // current, and the list should never be shorter than 23 at this point
        let mut new_start = marbles.split_off(marbles.len()-7);
        new_start.append(marbles);
        let score = next_marble + new_start.pop_front().unwrap();
        *marbles = new_start;
        return score
    }
    let mut new_start = marbles.split_off(2 % marbles.len());
    new_start.append(marbles);
    new_start.push_front(next_marble);
    *marbles = new_start;
    0
}


fn play(num_players: usize, last_marble: usize) -> u64 {
    let mut player_scores = vec![0u64; num_players as usize];
    let mut marbles: LinkedList<u64> = LinkedList::new();
    marbles.push_back(0);
    let mut curr_player = 0usize;
    for next_marble in 1..last_marble {
        player_scores[curr_player] += place(&mut marbles, next_marble as u64);
        curr_player = (curr_player + 1) % num_players;
    }
    if let Some(max_score) = player_scores.iter().max() {
        return *max_score;
    }
    0
}

fn main() {
    let input: Vec<Vec<usize>> = read_lines::<String>("input.txt")
        .unwrap()
        .iter()
        .map(|l| l.split(" ")
            .into_iter()
            .map(|x| x.chars().filter(|y| y.is_numeric()).collect::<String>())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect())
        .collect();

    // Handle as list to be able to test all examples as well.
    for line in input {
        let (num_players, last_marble) = (line[0] as usize, line[1]);
        println!("part1: {}", play(num_players, last_marble));
        println!("part2: {}", play(num_players, last_marble*100));
    }
}
