mod intcode;

static INPUT: &str = include_str!("../input.txt");

use intcode::VM;

fn part_one() {
    let mut vm = VM::default();
    vm.load_memory_from_tape(INPUT);
    vm.put_input(1);
    vm.run_until_terminated();

    println!("Part one: {:?}", vm.get_output());
}

fn part_two() {
    let mut vm = VM::default();
    vm.load_memory_from_tape(INPUT);
    vm.put_input(2);
    vm.run_until_terminated();

    println!("Part two: {:?}", vm.get_output());
}

fn main() {
    part_one();
    part_two();
}
