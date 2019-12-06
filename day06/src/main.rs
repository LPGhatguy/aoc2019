use std::collections::{HashMap, HashSet, VecDeque};

fn compute_cost<'a>(
    orbits: &HashMap<&'a str, &'a str>,
    costs: &mut HashMap<&'a str, usize>,
    subject: &'a str,
) -> usize {
    match costs.get(&subject) {
        Some(&cost) => cost,
        None => match orbits.get(subject) {
            Some(&next) => {
                let cost = 1 + compute_cost(orbits, costs, next);
                costs.insert(subject, cost);
                cost
            }
            None => 0,
        },
    }
}

fn part_one() {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in include_str!("../input.txt").lines() {
        let mut sides = line.split(')');
        let orbitee = sides.next().unwrap();
        let orbiter = sides.next().unwrap();
        orbits.insert(orbiter, orbitee);
    }

    let mut costs: HashMap<&str, usize> = HashMap::new();

    let sum: usize = orbits
        .keys()
        .map(|key| compute_cost(&orbits, &mut costs, *key))
        .sum();

    println!("Part one: {}", sum);
}

#[derive(Debug, Default)]
struct Body {
    parent: Option<&'static str>,
    children: Vec<&'static str>,
}

fn part_two() {
    let mut bodies: HashMap<&'static str, Body> = HashMap::new();

    for line in include_str!("../input.txt").lines() {
        let mut sides = line.split(')');
        let inner = sides.next().unwrap();
        let outer = sides.next().unwrap();

        bodies.entry(inner).or_default().children.push(outer);
        bodies.entry(outer).or_default().parent = Some(inner);
    }

    let you_orbiting = bodies["YOU"].parent.unwrap();
    let san_orbiting = bodies["SAN"].parent.unwrap();

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    to_visit.push_back((you_orbiting, 0));

    while let Some((now, dist)) = to_visit.pop_front() {
        visited.insert(now);

        if now == san_orbiting {
            println!("Part two: {}", dist);
            break;
        }

        let body = &bodies[now];

        for connected in body.children.iter().copied().chain(body.parent) {
            if !visited.contains(connected) {
                to_visit.push_back((connected, dist + 1));
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
