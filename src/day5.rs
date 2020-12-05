use std::fs::read_to_string;

// https://www.reddit.com/r/rust/comments/k71r9n/advent_of_code_2020_day_5/

const INPUT_FILENAME: &str = "inputs/input5";

pub fn solve() {
    println!("Part 1: {}", part01(INPUT_FILENAME));
    println!("Part 2: {}", part02(INPUT_FILENAME));
}

fn part01(file_name: &str) -> u32 {
    read_to_string(file_name).unwrap().lines().map(|line| get_id(line)).max().unwrap()
}

fn part02(file_name: &str) -> u32 {
    let mut ids: Vec<u32> = read_to_string(file_name).unwrap().lines().map(|line| get_id(line)).collect();
    ids.sort();
    ids.windows(2).find(|x| x[0] + 1 != x[1]).unwrap()[0] + 1
}

fn get_id(line: &str) -> u32 {
    line.char_indices().fold(0, |acc, (idx, char)|
        if char == 'B' || char == 'R' {
            acc + 2u32.pow((9 - idx) as u32)
        } else {
            acc
        })
}