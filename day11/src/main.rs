use std::fs::read_to_string;

fn calc_grid_value(x: &i64, y: &i64, serial: &i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = ((rack_id * y) + serial) * rack_id;
    power_level = power_level % 1000;
    power_level = (power_level - power_level % 100) / 100;
    power_level - 5
}

fn calc_grid_from_serial(grid_serial: &i64) -> Vec<Vec<i64>> {
    let mut cells = vec![vec![-100i64; 302]; 302];
    for x in 1..301 {
        for y in 1..301 {
            cells[x][y] = calc_grid_value(&(x as i64), &(y as i64), grid_serial);
        }
    }
    cells
}

fn calc_subgrib_power(cells: &Vec<Vec<i64>>, x: &i64, y: &i64, size: &i64) -> i64 {
    let mut power = 0;
    for dx in 0..*size {
        for dy in 0..*size {
            power += cells[(x+dx) as usize][(y+dy) as usize];
        }
    }
    power
}

fn main() {
    let grid_serial = read_to_string("input.txt").unwrap().parse::<i64>().unwrap();
    let cells = calc_grid_from_serial(&grid_serial);

    let mut max_power = -10*9i64;
    let (mut cx, mut cy) = (0i64, 0i64);
    for x in 1..299 {
        for y in 1..299 {
            let subgrid_power = calc_subgrib_power(&cells, &x, &y, &3);
            if subgrid_power > max_power {
                max_power = subgrid_power;
                cx = x;
                cy = y;
            }
        }
    }

    println!("part1: {},{}", cx, cy);

    let mut best_size = 0;
    for x in 1..301 {
        for y in 1..301 {
            let lim = if x > y {x} else {y};
            for size in 1..(301-lim) {
                let power = calc_subgrib_power(&cells, &x, &y, &size);
                if power > max_power {
                    max_power = power;
                    cx = x;
                    cy = y;
                    best_size = size;
                } else if size > 16 {
                    // Statiscically there are more negative numbers than positive
                    // If the value isn't increasing anymore it is probably not the
                    // best spot
                    break
                }
            }
        }
    }
    println!("part2: {},{},{}", cx, cy, best_size);
}
