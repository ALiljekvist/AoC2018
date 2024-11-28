use std::collections::{HashMap, HashSet};

use aoc_tools::input::input::read_lines;

#[derive(Debug)]
struct Cart {
    row: usize,
    col: usize,
    dir: i8, // right=0, then + 1 for each direction clockwise
    turn: i8,
    crashed: bool,
}

impl Cart {
    fn ride(&mut self, paths: &Vec<Vec<char>>) {
        // Move in the direction we're heading
        match self.dir {
            0 => {self.col += 1},
            1 => {self.row += 1},
            2 => {self.col -= 1},
            3 => {
                self.row -= 1

            },
            _ => {}
        }
        // Check if we should turn
        match paths[self.row][self.col] {
            '\\' => {
                // regular turn
                if self.dir % 2 == 0 {
                    self.dir = add_mod_four(self.dir, 1);
                } else {
                    self.dir = add_mod_four(self.dir, -1);
                }
            }
            '/' => {
                // regular turn
                if self.dir % 2 == 1 {
                    self.dir = add_mod_four(self.dir, 1);
                } else {
                    self.dir = add_mod_four(self.dir, -1);
                }
            }
            '+' => {
                // Intersection, make the correct turn and then
                // update which turn to make the next time
                self.dir = add_mod_four(self.dir, self.turn);
                match self.turn {
                    0 => {self.turn = 1}
                    1 => {self.turn = 3}
                    3 => {self.turn = 0}
                    _ => {}
                    }
                }
            _ => {}
        }
    }
}

fn add_mod_four(a: i8, b: i8) -> i8 {
    let mut c = (a + b) % 4;
    if c < 0 {
        c += 4
    }
    c
}

fn parse_carts(paths: &mut Vec<Vec<char>>) -> Vec<Cart> {
    let mut carts: Vec<Cart> = Vec::new();
    for i in 0..paths.len() {
        for j in 0..paths[i].len() {
            let dir = match paths[i][j] {
                '>' => {
                    paths[i][j] = '-';
                    0
                }
                'v' => {
                    paths[i][j] = '|';
                    1
                }
                '<' => {
                    paths[i][j] = '-';
                    2
                }
                '^' => {
                    paths[i][j] = '|';
                    3
                }
                _ => {-1}
            };
            if dir < 0 {
                continue;
            }
            carts.push(Cart { row: i, col: j, dir: dir , turn: 3, crashed: false});
        }
    }
    carts
}

fn check_for_crashes(carts: &Vec<Cart>) -> Option<(usize, usize)> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for cart in carts.iter() {
        if cart.crashed {
            continue;
        }
        if seen.contains(&(cart.row, cart.col)) {
            return Some((cart.row, cart.col));
        }
        seen.insert((cart.row, cart.col));
    }
    None
}

fn ride_until_one_left(
    paths: &Vec<Vec<char>>,
    carts: &mut Vec<Cart>
)
    -> (usize, usize) {
        // let mut turn = 0;
    let mut first_crashsite: Option<(usize, usize)> = None;
    while carts.len() > 1 {
        // turn += 1;
        // println!("Turn {}", turn);
        // printMap(paths, carts);
        carts.sort_unstable_by_key(|x| (x.row, x.col));
        for i in 0..carts.len() {
            if carts[i].crashed {
                continue
            }
            carts[i].ride(paths);
            if let Some((r, c)) = check_for_crashes(carts) {
                if first_crashsite == None {
                    println!("part1: {},{}", c, r);
                    first_crashsite = Some((r, c));
                }
                for j in 0..carts.len() {
                    if carts[j].row == r && carts[j].col == c {
                        carts[j].crashed = true;
                    }
                }
            }
        }
        carts.retain(|c| !c.crashed);
    }
    if carts.len() == 0 {
        return (0,0);
    }
    (carts[0].row, carts[0].col)
}

// Used for debugging, uncomment to print the map each turn
// fn printMap(paths: &Vec<Vec<char>>, carts: &Vec<Cart>) {
//     let mut cartMap: HashMap<(usize, usize),i8> = HashMap::new();
//     for c in carts {
//         cartMap.insert((c.row, c.col), c.dir);
//     }
//     for (r,row) in paths.iter().enumerate() {
//         for (c, ch) in row.iter().enumerate() {
//             if let Some(dir) = cartMap.get(&(r, c)) {
//                 match dir {
//                     0 => {print!(">");}
//                     1 => {print!("v");}
//                     2 => {print!("<");}
//                     3 => {print!("^");}
//                     _ => {}
//                 }
//                 continue;
//             }
//             print!("{}", ch);
//         }
//         print!("\n");
//     }
// }

fn main() {
    let mut paths: Vec<Vec<char>> = read_lines::<String>("input.txt")
            .unwrap()
            .into_iter()
            .map(|x| x.chars()
                .into_iter()
                .collect())
            .collect();
    let mut carts: Vec<Cart> = parse_carts(&mut paths);
    let (row, col) = ride_until_one_left(&paths, &mut carts);
    println!("part2: {},{}", col, row);
    // 24,111 is wrong
}
