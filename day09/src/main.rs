mod intcode;

static INPUT: &str = include_str!("../input.txt");

use intcode::VM;

fn main() {
    let mut vm = VM::default();
    vm.load_memory_from_tape(INPUT);
    vm.put_input(1);
    vm.run_until_terminated();

    println!("{:?}", vm.output);
}
