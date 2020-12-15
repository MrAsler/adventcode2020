use crate::get_result_i64;
use std::collections::HashMap;
use std::ops::Neg;

// https://adventofcode.com/2020/day/14
// https://www.reddit.com/r/rust/comments/kcrbxw/advent_of_code_2020_day_14/

const INPUT_FILENAME: &str = "inputs/input14";

pub fn solve() {
    get_result_i64(1, part01, INPUT_FILENAME);
    get_result_i64(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> i64 {
    let mut mask_1: i64 = 0;
    let mut mask_0: i64 = 0;
    let mut memory: HashMap<usize, i64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let bitmask = &line[line.find("= ").unwrap() + 2..];
            mask_1 = i64::from_str_radix(&bitmask.replace('X', "0"), 2).unwrap();
            mask_0 = i64::from_str_radix(&bitmask.replace('X', "1"), 2).unwrap();
        } else {
            let memory_address = &line[line.find('[').unwrap() + 1..line.find(']').unwrap()].parse::<usize>().unwrap();
            let mut value = *&line[line.find("= ").unwrap() + 2..].parse::<i64>().unwrap();
            value = (value | mask_1) & mask_0;
            if value & (1 << 36) != 0 {
                value = value.neg();
            }
            &memory.insert(*memory_address, value);
        }
    }

    memory.values().sum()
}

fn part02(input: String) -> i64 {
    let mut bitmask: i64 = 0;
    let mut floating_bits: Vec<usize> = Vec::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let bitmask_str = &line[line.find("= ").unwrap() + 2..];
            bitmask = i64::from_str_radix(&bitmask_str.replace('X', "0"), 2).unwrap();
            floating_bits = bitmask_str.char_indices().filter(|(idx, char)| *char == 'X').map(|(idx, _)| 35 - idx).collect();
        } else {
            let mut memory_address = *&line[line.find('[').unwrap() + 1..line.find(']').unwrap()].parse::<i64>().unwrap();
            let value = *&line[line.find("= ").unwrap() + 2..].parse::<i64>().unwrap();
            memory_address = memory_address | bitmask;
            recursive(&floating_bits, &mut memory, memory_address, value);
        }
    }

    memory.values().sum()
}

fn recursive(floating_bits: &[usize], memory: &mut HashMap<i64, i64>, memory_address: i64, value: i64) {
    if floating_bits.is_empty() {
        return;
    }

    memory.insert(memory_address, value);
    recursive(&floating_bits[1..], memory, memory_address, value);
    let new_address = memory_address ^ (1 << floating_bits[0]);
    memory.insert(new_address, value);
    recursive(&floating_bits[1..], memory, new_address, value);
}
