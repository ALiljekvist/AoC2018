use aoc_tools::input::*;
use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let ids: Vec<String> = input::read_lines("input.txt").unwrap();
    let mut double_cnt = 0i64;
    let mut tripple_cnt = 0i64;
    for id in ids.iter() {
        let mut let_cnt: HashMap<char, i64> = HashMap::new();
        for c in id.chars() {
            if let Some(cnt) = let_cnt.get_mut(&c) {
                *cnt += 1;
                continue;
            }
            let_cnt.insert(c, 1);
        }
        let vals: Vec<i64> = let_cnt.into_values().collect();
        if vals.contains(&2) {double_cnt += 1}
        if vals.contains(&3) {tripple_cnt += 1}
    }
    println!("part1: {}", double_cnt * tripple_cnt);

    let mut found = false;
    for i in 0..ids.len() {
        let word1: Vec<char> = ids[i].chars().collect();
        for j in i+1..ids.len() {
            let word2: Vec<char> = ids[j].chars().collect();
            let mut diff_cnt = 0u8;
            let res: Vec<char> = zip(word1.clone(), word2)
            .into_iter()
            .filter(|(x, y)| {
                if x != y {
                    diff_cnt+=1;
                }
                x == y
            })
            .map(|(_, y)| y)
            .collect();
            if diff_cnt == 1 {
                println!("part2: {}", res.into_iter().collect::<String>());
                found = true;
                break
            }
        }
        if found {
            break
        }
    }
}