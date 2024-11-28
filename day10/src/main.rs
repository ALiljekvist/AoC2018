use aoc_tools::input::*;
use input::read_lines;
use std::str::FromStr;

#[derive(Debug)]
struct ParseDotError{}

#[derive(Debug, Clone)]
struct Dot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Dot {
    fn travel(&mut self, t: &i64) {
        self.x += self.dx * t;
        self.y += self.dy * t;
    }

    fn hit(&self, x: &i64, y:&i64) -> bool {
        return &self.x == x && &self.y == y
    }
}

impl FromStr for Dot {
    type Err = ParseDotError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let stuff: Vec<i64> = s.split(" ")
            .into_iter()
            .map(|x| x.chars()
                .into_iter()
                .filter(|c| c.is_numeric() || c == &'-')
                .collect::<String>()
            )
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(Dot { x: stuff[0], y: stuff[1], dx: stuff[2], dy: stuff[3] })
    }
}

fn print_particles(particles: &Vec<Dot>) {
    let p1 = particles[0].clone();
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (p1.x, p1.x, p1.y, p1.y);
    for p in particles.iter() {
        if p.x < x_min {
            x_min = p.x;
        }
        if  p.x > x_max {
            x_max = p.x;
        }
        if p.y < y_min {
            y_min = p.y;
        }
        if  p.y > y_max {
            y_max = p.y;
        }
    }

    // Assume text will appear in X-direction (because example did)
    for y in y_min..y_max+1 {
        for x in x_min..x_max+1 {
            let mut hit = false;
            for p in particles.iter() {
                if p.hit(&x, &y) {
                    hit = true;
                    break
                }
            }
            if hit {
                print!("#");
                continue;
            }
            print!(" ");
        }
        print!("\n");
    }
}

fn calc_min_span_len(particles: &Vec<Dot>) -> i64 {
    let mut max_x_dist = 0i64;
    let mut max_y_dist = 0i64;
    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let x_dist = (particles[i].x - particles[j].x).abs();
            if x_dist > max_x_dist {
                max_x_dist = x_dist;
            }
            let y_dist = (particles[i].y - particles[j].y).abs();
            if y_dist > max_y_dist {
                max_y_dist = y_dist;
            }
        }
    }
    if max_x_dist < max_y_dist {
        return max_x_dist
    }
    max_y_dist
}

fn main() {
    let mut particles = read_lines::<Dot>("input.txt").unwrap();
    let mut t = 0i64;
    // Calculate the minimum distance and the maximum total speed and set dt dynamically
    // based on those two numbers.
    // (This could be further optimized by calculating the distances and speeds in
    // in each direction, or even better calculate when different lines meet to see
    // what time steps would be interesting to look at)
    let mut dist = calc_min_span_len(&particles);
    let max_speed = particles.iter().map(|x| x.dx + x.dy).max().unwrap();
    let mut dt = dist/max_speed;
    loop {
        for p in particles.iter_mut() {
            p.travel(&dt);
        }
        t += dt;
        let max_min_dist = calc_min_span_len(&particles);
        if max_min_dist < dist {
            dist = max_min_dist;
            // update dt
            dt = dist / max_speed;
        }
        if dist < 10*max_speed {
            // we're close, go one step at a time
            dt = 1;
        }
        // Assume the text is somewhat close together the correct size (will not work for example)
        if max_min_dist < 10 {
            // moved away from center of where the particles meet
            break
        }
    }
    print_particles(&particles);
    println!("\npart2: {}", t);
}
