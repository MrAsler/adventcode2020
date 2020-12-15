use parse_display::{Display as PDisplay, FromStr as PFromStr};
use crate::get_result_i32;
use std::collections::HashSet;

// https://adventofcode.com/2020/day/8
// https://www.reddit.com/r/rust/comments/k90t1l/advent_of_code_2020_day_8/

const INPUT_FILENAME: &str = "inputs/input08";

pub fn solve() {
    get_result_i32(1, part01, INPUT_FILENAME);
    get_result_i32(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> i32 {
    Computer::new(input).process().unwrap_err()
}

fn part02(input: String) -> i32 {
    Computer::new(input).fix()
}

struct Computer {
    ops: Vec<Instruction>,
    global_acc: i32,
    pointer: i32,
}

impl Computer {
    fn new(input: String) -> Computer {
        Computer {
            ops: input.lines().map(|line| line.parse().unwrap()).collect(),
            global_acc: 0,
            pointer: 0,
        }
    }

    fn fix(&mut self) -> i32 {
        for i in 0..self.ops.len() {
            let instr = &self.ops[i];
            if instr.op == Operation::Jmp || instr.op == Operation::Nop {
                let new_instr: Instruction;
                let backup: Instruction;

                if instr.op == Operation::Jmp {
                    new_instr = Instruction { op: Operation::Nop, value: instr.value };
                    backup = Instruction { op: Operation::Jmp, value: instr.value };
                } else {
                    new_instr = Instruction { op: Operation::Jmp, value: instr.value };
                    backup = Instruction { op: Operation::Nop, value: instr.value };
                }

                self.ops[i] = new_instr;
                self.pointer = 0;
                self.global_acc = 0;

                match self.process() {
                    Ok(val) => return val,
                    Err(_) => {}
                }

                self.ops[i] = backup;
            }
        }

        return -1;
    }

    fn process(&mut self) -> Result<i32, i32> {
        let mut processed_instructions = HashSet::new();

        loop {
            if self.pointer == self.ops.len() as i32 {
                return Ok(self.global_acc);
            }

            if processed_instructions.contains(&self.pointer) {
                return Err(self.global_acc);
            }

            let instruction = &self.ops[self.pointer as usize];
            processed_instructions.insert(self.pointer);
            match &instruction.op {
                Operation::Acc => self.global_acc += instruction.value,
                Operation::Jmp => self.pointer += instruction.value - 1,
                Operation::Nop => {}
            }

            self.pointer += 1;
        }
    }
}


#[derive(PDisplay, PFromStr, Debug)]
#[display("{op} {value}")]
struct Instruction {
    op: Operation,
    value: i32,
}

#[derive(PDisplay, PFromStr, PartialEq, Debug, Clone)]
enum Operation {
    #[display("acc")]
    Acc,
    #[display("jmp")]
    Jmp,
    #[display("nop")]
    Nop,
}