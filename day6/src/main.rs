use aoc_tools::input::*;
use input::read_lines;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64
}

impl Coord {
    fn l1_dist(&self, x: i64, y: i64) -> i64 {
        return (self.x-x).abs() + (self.y-y).abs()
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let vals: Vec<i64> = s.split(", ").map(|x| x.parse().unwrap()).collect();
        Ok(Coord { x: vals[0], y: vals[1] })
    }
}

fn main() {
    let coords = read_lines::<Coord>("input.txt").unwrap();
    // Find the corneres of the smallest rectangle enclosing the points.
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0i64, 0i64, 0i64, 0i64);
    for coord in coords.iter() {
        if coord.x < x_min {
            x_min = coord.x
        } else if coord.x > x_max {
            x_max = coord.x
        }
        if coord.y < y_min {
            y_min = coord.y
        } else if coord.y > y_max {
            y_max = coord.y
        }
    }
    // Iterate through all coords in the rectangle and keep a sum for each
    // coord, and disqualify any coord that owns a point on the border (infinite)
    let mut areas: HashMap<usize, u64> = HashMap::new();
    let mut disqualified: HashMap<usize, bool> = HashMap::new();
    for i in x_min..x_max+1 {
        for j in y_min..y_max+1 {
            let mut min_dist = x_max+1-x_min;
            let mut clostest_coord = -1i64;
            for ind in 0..coords.len() {
                let d = coords[ind].l1_dist(i, j);
                if d < min_dist {
                    min_dist = d;
                    clostest_coord = ind as i64;
                } else if d == min_dist {
                    clostest_coord = -1;
                } 
            }
            if clostest_coord == -1 {
                continue
            }
            let a = areas.entry(clostest_coord as usize).or_insert(0);
            *a += 1;
            if i == x_min || i == x_max || j == y_min || j == y_max {
                disqualified.insert(clostest_coord as usize, true);
            }
        }
    }
    // Look through areas and select valid coord with smallest area
    let mut largest_area = 0u64;
    for (coord, area) in areas.iter() {
        if disqualified.contains_key(coord) {
            continue
        }
        if area > &largest_area {
            largest_area = *area;
        }
    }
    println!("part1: {}", largest_area);

    let mut second_region = 0i64;
    for i in x_min..x_max+1 {
        for j in y_min..y_max+1 {
            let tot_dist: i64 = coords.iter()
                .map(|x| x.l1_dist(i, j))
                .sum();
            if tot_dist < 10000 {
                second_region += 1;
            }
        }
    }
    println!("part2: {}", second_region);
}
