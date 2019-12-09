use std::collections::VecDeque;

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
pub struct VM {
    pub pc: usize,
    pub pc_checkpoint: usize,
    pub memory: Vec<i32>,
    pub input: VecDeque<i32>,
    pub output: VecDeque<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Terminated,
    NeedsInput,
    SentOutput,
}

impl VM {
    pub fn load(&self, mode: u8, value: i32) -> i32 {
        match mode {
            MODE_POS => self.memory[value as usize],
            MODE_IMM => value,
            _ => panic!("illegal operand mode"),
        }
    }

    pub fn put_input(&mut self, value: i32) {
        self.input.push_back(value);
    }

    pub fn get_output(&mut self) -> i32 {
        self.output.pop_front().unwrap()
    }

    pub fn load_memory(&mut self, memory: impl Into<Vec<i32>>) {
        self.memory = memory.into();
    }

    pub fn decode_tape(input: &str) -> Vec<i32> {
        input.split(',').map(|v| v.parse().unwrap()).collect()
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

    fn checkpoint(&mut self) {
        self.pc_checkpoint = self.pc;
    }

    fn rewind(&mut self) {
        self.pc = self.pc_checkpoint;
    }

    pub fn run_until_terminated(&mut self) {
        loop {
            match self.run_partial() {
                Outcome::Terminated => break,
                Outcome::NeedsInput => panic!("System starved for input"),
                Outcome::SentOutput => {}
            }
        }
    }

    pub fn run_partial(&mut self) -> Outcome {
        loop {
            self.checkpoint();
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

                    match self.input.pop_front() {
                        Some(value) => self.store(out, value),
                        None => {
                            self.rewind();
                            return Outcome::NeedsInput;
                        }
                    }
                }
                OP_OUT => {
                    let a = self.arg_value(mode1);

                    self.output.push_back(a);
                    return Outcome::SentOutput;
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
                OP_END => {
                    self.rewind();
                    break;
                }
                _ => panic!("illegal instruction"),
            }
        }

        Outcome::Terminated
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
