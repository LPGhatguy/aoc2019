use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

fn travel(wire: &[(i32, i32)], mut callback: impl FnMut((i32, i32))) {
    let mut pos = (0, 0);

    for delta in wire {
        let goal = (pos.0 + delta.0, pos.1 + delta.1);

        // normalize that would fail if more than one component were nonzero
        let step = (delta.0.signum(), delta.1.signum());

        while pos != goal {
            pos = (pos.0 + step.0, pos.1 + step.1);
            callback(pos);
        }
    }
}

fn get_wires() -> [Vec<(i32, i32)>; 2] {
    let mut wires = INPUT.lines().map(|line| {
        line.split(",")
            .map(|inst| {
                let mag: i32 = inst[1..].parse().unwrap();

                match inst.chars().next().unwrap() {
                    'R' => (mag, 0),
                    'L' => (-mag, 0),
                    'U' => (0, mag),
                    'D' => (0, -mag),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>()
    });

    [wires.next().unwrap(), wires.next().unwrap()]
}

fn main() {
    let [first_wire, second_wire] = get_wires();

    let mut board = HashMap::new();

    let mut num_steps = 1;
    travel(&first_wire, |pos| {
        board.insert(pos, num_steps);
        num_steps += 1;
    });

    let mut closest_dist = i32::max_value();
    let mut fewest_steps = u32::max_value();

    let mut num_steps = 1;
    travel(&second_wire, |pos| {
        if let Some(&first_steps) = board.get(&pos) {
            closest_dist = closest_dist.min(pos.0.abs() + pos.1.abs());

            let total_steps = num_steps + first_steps;
            fewest_steps = fewest_steps.min(total_steps);
        }
        num_steps += 1;
    });

    println!("Part one: {}", closest_dist);
    println!("Part two: {}", fewest_steps);
}
