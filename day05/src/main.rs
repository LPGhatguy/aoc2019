use std::collections::VecDeque;

static INPUT: &str = include_str!("../input.txt");

const OP_ADD: u8 = 1;
const OP_MUL: u8 = 2;
const OP_INN: u8 = 3;
const OP_OUT: u8 = 4;
const OP_JIT: u8 = 5; // jump if true
const OP_JIF: u8 = 6; // jump if false
const OP_CML: u8 = 7; // compare less
const OP_CME: u8 = 8; // compare equal
const OP_END: u8 = 99;

const MODE_POS: u8 = 0;
const MODE_IMM: u8 = 1;

#[derive(Default)]
struct VM {
    pc: usize,
    memory: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
}

impl VM {
    fn load(&self, mode: u8, value: i32) -> i32 {
        match mode {
            MODE_POS => self.memory[value as usize],
            MODE_IMM => value,
            _ => panic!("illegal operand mode"),
        }
    }

    fn store(&mut self, ptr: i32, value: i32) {
        self.memory[ptr as usize] = value;
    }

    fn arg_value(&mut self, mode: u8) -> i32 {
        let value = self.memory[self.pc];
        self.pc += 1;
        self.load(mode, value)
    }

    fn arg_raw(&mut self) -> i32 {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }

    fn run(&mut self) {
        loop {
            let inst = self.arg_raw();
            let (op, mode1, mode2, _mode3) = decode_instruction(inst);

            match op {
                OP_ADD => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    self.store(out, a + b);
                }
                OP_MUL => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    self.store(out, a * b);
                }
                OP_INN => {
                    let out = self.arg_raw();

                    let value = self.input.pop_front().unwrap();
                    self.store(out, value);
                }
                OP_OUT => {
                    let a = self.arg_value(mode1);

                    self.output.push(a);
                }
                OP_JIT => {
                    let cond = self.arg_value(mode1);
                    let dest = self.arg_value(mode2);

                    if cond != 0 {
                        self.pc = dest as usize;
                    }
                }
                OP_JIF => {
                    let cond = self.arg_value(mode1);
                    let dest = self.arg_value(mode2);

                    if cond == 0 {
                        self.pc = dest as usize;
                    }
                }
                OP_CML => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    if a < b {
                        self.store(out, 1);
                    } else {
                        self.store(out, 0);
                    }
                }
                OP_CME => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    if a == b {
                        self.store(out, 1);
                    } else {
                        self.store(out, 0);
                    }
                }
                OP_END => break,
                _ => panic!("illegal instruction"),
            }
        }
    }
}

fn decode_instruction(instruction: i32) -> (u8, u8, u8, u8) {
    let mut remaining = instruction;
    let op = instruction % 100;
    remaining /= 100;

    let mode1 = remaining % 10;
    remaining /= 10;

    let mode2 = remaining % 10;
    remaining /= 10;

    let mode3 = remaining % 10;

    (op as u8, mode1 as u8, mode2 as u8, mode3 as u8)
}

fn load_memory() -> Vec<i32> {
    INPUT.split(',').map(|v| v.parse().unwrap()).collect()
}

fn part_one() {
    let mut vm = VM::default();

    vm.memory = load_memory();
    vm.input.push_back(1);
    vm.run();

    println!("Part one: {:?}", vm.output[vm.output.len() - 1]);
}

fn part_two() {
    let mut vm = VM::default();

    vm.memory = load_memory();
    vm.input.push_back(5);
    vm.run();

    println!("Part two: {:?}", vm.output[0]);
}

fn main() {
    env_logger::init();

    part_one();
    part_two();
}
