#![allow(dead_code)]

use std::collections::VecDeque;

const OP_ADD: u8 = 1;
const OP_MUL: u8 = 2;
const OP_INN: u8 = 3;
const OP_OUT: u8 = 4;
const OP_JIT: u8 = 5; // jump if true
const OP_JIF: u8 = 6; // jump if false
const OP_CML: u8 = 7; // compare less
const OP_CME: u8 = 8; // compare equal
const OP_ARB: u8 = 9; // add to relative base
const OP_END: u8 = 99;

const MODE_POS: u8 = 0;
const MODE_IMM: u8 = 1;
const MODE_REL: u8 = 2;

#[derive(Default)]
pub struct VM {
    pub pc: usize,
    pub pc_checkpoint: usize,
    pub rb: usize,
    pub memory: Vec<i64>,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Terminated,
    NeedsInput,
    SentOutput,
}

impl VM {
    pub fn load_memory(&mut self, memory: impl Into<Vec<i64>>) {
        self.memory = memory.into();
    }

    pub fn load_memory_from_tape(&mut self, tape: &str) {
        let memory = Self::decode_tape(tape);
        self.load_memory(memory);
    }

    pub fn put_input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn get_output(&mut self) -> i64 {
        self.output.pop_front().unwrap()
    }

    pub fn decode_tape(input: &str) -> Vec<i64> {
        input.split(',').map(|v| v.parse().unwrap()).collect()
    }

    fn load(&self, mode: u8, value: i64) -> i64 {
        match mode {
            MODE_POS => self.read_ptr(value as usize),
            MODE_IMM => value,
            MODE_REL => self.read_ptr((self.rb as i64 + value) as usize),
            _ => panic!("illegal operand mode"),
        }
    }

    fn read_ptr(&self, ptr: usize) -> i64 {
        self.memory.get(ptr).copied().unwrap_or(0)
    }

    fn store(&mut self, ptr: i64, ptr_mode: u8, value: i64) {
        let ptr = match ptr_mode {
            MODE_POS => ptr as usize,
            MODE_REL => (self.rb as i64 + ptr) as usize,
            _ => panic!("illegal pointer mode {}", ptr_mode),
        };

        if ptr >= self.memory.len() {
            self.memory.resize(ptr + 1, 0);
        }

        self.memory[ptr as usize] = value;
    }

    fn arg_value(&mut self, mode: u8) -> i64 {
        let value = self.read_ptr(self.pc);
        self.pc += 1;
        self.load(mode, value)
    }

    fn arg_raw(&mut self) -> i64 {
        let value = self.read_ptr(self.pc);
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
            let (op, mode1, mode2, mode3) = decode_instruction(inst);

            match op {
                OP_ADD => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    self.store(out, mode3, a + b);
                }
                OP_MUL => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    self.store(out, mode3, a * b);
                }
                OP_INN => {
                    let out = self.arg_raw();

                    match self.input.pop_front() {
                        Some(value) => self.store(out, mode1, value),
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
                        self.store(out, mode3, 1);
                    } else {
                        self.store(out, mode3, 0);
                    }
                }
                OP_CME => {
                    let a = self.arg_value(mode1);
                    let b = self.arg_value(mode2);
                    let out = self.arg_raw();

                    if a == b {
                        self.store(out, mode3, 1);
                    } else {
                        self.store(out, mode3, 0);
                    }
                }
                OP_ARB => {
                    let adjust = self.arg_value(mode1);

                    self.rb = (self.rb as i64 + adjust) as usize;
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

fn decode_instruction(instruction: i64) -> (u8, u8, u8, u8) {
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
