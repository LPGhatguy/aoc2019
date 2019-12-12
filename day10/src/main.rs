use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
};

static INPUT: &str = include_str!("../input.txt");

#[derive(Default)]
struct Field {
    // space is mostly empty
    occupied: HashSet<(usize, usize)>,
}

impl Field {
    fn load(&mut self, board: &str) {
        for (y, line) in board.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                match cell {
                    '#' => {
                        self.occupied.insert((x, y));
                    }
                    '.' => {
                        // this space intentionally left blank
                    }
                    _ => panic!("Bad map cell {}", cell),
                }
            }
        }
    }
}

// thanks, euclid
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let (next_a, next_b) = (b, a % b);

        a = next_a;
        b = next_b;
    }

    a
}

// this is kind of like a fraction simplifcation function but bad
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

    // there might be a better way to preserve our signs here
    if num.signum() != new_num.signum() {
        new_num = -new_num;
    }

    if denom.signum() != new_denom.signum() {
        new_denom = -new_denom;
    }

    (new_num, new_denom)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sightline(i64, i64);

impl Sightline {
    // sightlines are sorted by their angle, kind of
    fn ord_angle(&self) -> f32 {
        // make sure "up" is (0, -1)
        //
        // probably should use radians here, but 'degree' has more letters in
        // common with 'debug' than 'radian' does
        let angle = (self.1 as f32).atan2(self.0 as f32).to_degrees() + 90.0;

        if angle >= 0.0 {
            angle
        } else {
            angle + 360.0
        }
    }
}

impl From<(i64, i64)> for Sightline {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Ord for Sightline {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord_angle()
            .partial_cmp(&other.ord_angle())
            .expect("nan can't hurt you if you don't believe in it")
    }
}

impl PartialOrd for Sightline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut field = Field::default();
    field.load(INPUT);

    // the last time we picked a station, it was way out in the middle of
    // nowhere and couldn't see any asteroids. maybe we'll do better this time.
    let mut best_station = (99, 99);
    let mut best_score = 0;

    // a map of the sightlines of the best station position
    // the keys are sorted by their angle
    // the values are a list of the asteroids on that sight line
    let mut best_sightlines = BTreeMap::new();

    for (x1, y1) in field.occupied.iter().copied() {
        let mut occupied_sightlines = BTreeMap::new();

        // do you ever feel like you picked the wrong number types, but it's too
        // late to fix it?
        let x1 = x1 as i64;
        let y1 = y1 as i64;

        for (x2, y2) in field.occupied.iter().copied() {
            let x2 = x2 as i64;
            let y2 = y2 as i64;

            // we cannot see ourselves, maybe we should buy a mirror
            if x1 == x2 && y1 == y2 {
                continue;
            }

            // is this raycasting?
            let sightline = Sightline::from(simplify_sight((x2 - x1, y2 - y1)));
            let roids_on_line = occupied_sightlines.entry(sightline).or_insert(Vec::new());
            roids_on_line.push(((x2, y2), y2 - y1 + x2 - x1));
        }

        if occupied_sightlines.len() > best_score {
            best_score = occupied_sightlines.len();
            best_station = (x1, y1);
            best_sightlines = occupied_sightlines;
        }
    }

    println!("Station ({}, {}) is best", best_station.0, best_station.1);
    println!("Part one: {}", best_score);

    let total_vaporizable: usize = best_sightlines
        .values()
        .map(|asteroids| asteroids.len())
        .sum();

    // if this is wrong, then our sightline stuff is messed up
    assert_eq!(total_vaporizable, field.occupied.len() - 1);

    // we didn't want to pay the cost of keeping the asteroids sorted as we
    // built them up (what even is a heap)
    //
    // so we sort the cream of the crop roids here
    for roids in best_sightlines.values_mut() {
        roids.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    }

    // spin around in a circle and fire our laser
    let mut zapped_count = 0;
    loop {
        let mut zapped_this_time = 0;

        // it's incredible that this does what I wanted it to
        for roids in best_sightlines.values_mut() {
            // bzzt
            if let Some(((x, y), _)) = roids.pop() {
                zapped_this_time += 1;
                zapped_count += 1;

                if zapped_count == 200 {
                    println!("{}th roid zapped is ({}, {})", zapped_count, x, y);
                    println!("Part two: {}", x * 100 + y);
                }
            }
        }

        // no more hostiles detected
        if zapped_this_time == 0 {
            break;
        }
    }
}
