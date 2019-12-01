static INPUT: &str = include_str!("../input.txt");

fn fuel_for_mass(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn part_one() {
    let sum: u32 = INPUT
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(fuel_for_mass)
        .sum();

    println!("Part one: {}", sum);
}

fn fuel_for_fuel_and_mass(mass: u32) -> u32 {
    let mut total_mass = 0;
    let mut last_mass = mass;

    loop {
        let fuel = fuel_for_mass(last_mass);

        if fuel == 0 {
            break;
        }

        last_mass = fuel;
        total_mass += fuel;
    }

    total_mass
}

fn part_two() {
    let sum: u32 = INPUT
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(fuel_for_fuel_and_mass)
        .sum();

    println!("Part two: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
