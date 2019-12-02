static INPUT: &str = include_str!("../input.txt");

const OP_ADD: usize = 1;
const OP_MUL: usize = 2;
const OP_END: usize = 99;

fn run(memory: &mut [usize]) {
    let mut cursor = 0;

    loop {
        match memory[cursor] {
            OP_ADD => {
                let p_a = memory[cursor + 1];
                let p_b = memory[cursor + 2];
                let p_out = memory[cursor + 3];

                memory[p_out] = memory[p_a] + memory[p_b];

                cursor += 4;
            }
            OP_MUL => {
                let p_a = memory[cursor + 1];
                let p_b = memory[cursor + 2];
                let p_out = memory[cursor + 3];

                memory[p_out] = memory[p_a] * memory[p_b];

                cursor += 4;
            }
            OP_END => break,
            _ => panic!("illegal instruction"),
        }
    }
}

fn load() -> Vec<usize> {
    INPUT.split(',').map(|v| v.parse().unwrap()).collect()
}

fn part_one() {
    let mut memory = load();

    memory[1] = 12;
    memory[2] = 2;

    run(&mut memory);

    println!("Day one: {}", memory[0]);
}

fn part_two() {
    let base_memory = load();
    let goal_output = 19690720;

    for verb in 0..100 {
        for noun in 0..100 {
            let mut memory = base_memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            run(&mut memory);

            if memory[0] == goal_output {
                println!("Noun: {}, verb: {}", noun, verb);
                println!("Day two: {}", 100 * noun + verb);
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
