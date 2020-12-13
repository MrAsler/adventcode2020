use crate::get_result;

// https://adventofcode.com/2020/day/5
// https://www.reddit.com/r/rust/comments/k71r9n/advent_of_code_2020_day_5/

const INPUT_FILENAME: &str = "inputs/input5";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    input.lines().map(|line| get_id(line)).max().unwrap() as usize
}

fn part02(input: String) -> usize {
    let mut ids: Vec<u32> = input.lines().map(|line| get_id(line)).collect();
    ids.sort();
    (ids.windows(2).find(|x| x[0] + 1 != x[1]).unwrap()[0] + 1) as usize
}

fn get_id(line: &str) -> u32 {
    line.char_indices().fold(0, |acc, (idx, char)|
        if char == 'B' || char == 'R' {
            acc + 2u32.pow((9 - idx) as u32)
        } else {
            acc
        })
}