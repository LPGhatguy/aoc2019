use std::{
    collections::{HashMap, HashSet},
    fmt,
};

static INPUT: &str = include_str!("../input.txt");

struct Field {
    size: (usize, usize),
    occupied: HashSet<(usize, usize)>,
}

impl Field {
    fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            occupied: HashSet::new(),
        }
    }

    fn get(&self, pos: (usize, usize)) -> bool {
        self.occupied.contains(&pos)
    }

    fn set(&mut self, pos: (usize, usize)) {
        self.occupied.insert(pos);
    }

    fn load(&mut self, board: &str) {
        for (y, line) in board.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                match cell {
                    '#' => self.set((x, y)),
                    '.' => {}
                    _ => panic!("Bad map cell {}", cell),
                }
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if self.get((x, y)) {
                    write!(formatter, "#")?;
                } else {
                    write!(formatter, ".")?;
                }
            }

            writeln!(formatter)?;
        }

        Ok(())
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let (next_a, next_b) = (b, a % b);

        a = next_a;
        b = next_b;
    }

    a
}

fn simplify_sight((num, denom): (i64, i64)) -> (i64, i64) {
    if denom == 0 {
        return (num.signum(), 0);
    }

    if num == 0 {
        return (0, denom.signum());
    }

    let divisor = gcd(num, denom);

    let mut new_num = num / divisor;
    let mut new_denom = denom / divisor;

    if num.signum() != new_num.signum() {
        new_num = -new_num;
    }

    if denom.signum() != new_denom.signum() {
        new_denom = -new_denom;
    }

    (new_num, new_denom)
}

fn main() {
    let mut field = Field::new((21, 21));
    field.load(INPUT);

    let mut occupied_sightlines = HashSet::new();
    let mut best_station = (99, 99);
    let mut best_score = 0;

    for (x1, y1) in field.occupied.iter().copied() {
        let x1 = x1 as i64;
        let y1 = y1 as i64;

        for (x2, y2) in field.occupied.iter().copied() {
            let x2 = x2 as i64;
            let y2 = y2 as i64;

            if x1 == x2 && y1 == y2 {
                continue;
            }

            let sightline = simplify_sight((x2 - x1, y2 - y1));
            occupied_sightlines.insert(sightline);
        }

        println!(
            "({}, {}) can see {} asteroids",
            x1,
            y1,
            occupied_sightlines.len()
        );

        if occupied_sightlines.len() > best_score {
            best_score = occupied_sightlines.len();
            best_station = (x1, y1);
        }

        occupied_sightlines.clear();
    }

    println!(
        "Part one: {} from station ({}, {})",
        best_score, best_station.0, best_station.1
    );
}
