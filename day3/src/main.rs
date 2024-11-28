use std::{num::ParseIntError, str::FromStr};

use aoc_tools::input::*;
use input::read_lines;

#[derive(Debug, Clone)]
struct Claim {
    id: u64,
    w: usize,
    h: usize,
    offset_w: usize,
    offset_h: usize
}

impl Claim {
    fn overlaps(&self, other: &Claim) -> bool {
        for w in [other.offset_w, other.offset_w+other.w] {
            if self.offset_w > w || self.offset_w+self.w < w {
                continue
            }
            for h in [other.offset_h, other.offset_h+other.h] {
                if self.offset_h > h || self.offset_h+self.h < h {
                    continue
                }
                return true
            }
        }
        return false
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;
    fn from_str(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        let stuff: Vec<String> = value.split(" ").map(|x| x.to_string()).collect();
        let id: u64 = stuff[0][1..].parse()?;
        let offsets: Vec<usize> = stuff[2]
        .split(",")
        .map(|x| x.replace(":", "").parse().unwrap())
        .collect();
        let sizes: Vec<usize> = stuff[3]
        .split("x")
        .map(|x| x.parse().unwrap())
        .collect();
        Ok(Claim {
            id: id,
            w: sizes[0],
            h: sizes[1],
            offset_w: offsets[0],
            offset_h: offsets[1]
        })
    }
}

fn main() {
    let claims = read_lines::<Claim>("input.txt").unwrap();
    let mut fabric = vec![vec![0i64; 1000]; 1000];
    for claim in claims.iter() {
        for i in claim.offset_w..claim.offset_w+claim.w {
            for j in claim.offset_h..claim.offset_h+claim.h {
                fabric[i][j] += 1;
            }
        }
    }

    let mut doubled = 0i64;
    for row in fabric {
        for val in row {
            if val > 1 {
                doubled += 1 as i64;
            }
        }
    }
    println!("part1: {}", doubled);

    for i in 0..claims.len() {
        let mut overlapped = false;
        for j in 0..claims.len() {
            if i == j {
                continue
            }
            if claims[i].overlaps(&claims[j]) || claims[j].overlaps(&claims[i]) {
                overlapped = true;
                break
            }
        }
        if !overlapped {
            println!("part2: {}", claims[i].id);
            break
        }
    }
}
