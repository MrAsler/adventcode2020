use std::collections::HashSet;
use std::fs::read_to_string;

// https://www.reddit.com/r/rust/comments/k4hoyk/advent_of_code_2020_day_1/

const SUM_OBJECTIVE: i32 = 2020;
const INPUT_FILENAME: &str = "inputs/input1";

pub fn solve() {
    println!("Part 1: {}", part01(INPUT_FILENAME));
    println!("Part 2: {}", part02(INPUT_FILENAME));
}

fn part01(file_name: &str) -> i32 {
    let entries: HashSet<i32> = read_to_string(file_name).unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for entry in &entries {
        let other_entry = SUM_OBJECTIVE - entry;
        if entries.contains(&other_entry) {
            return entry * other_entry;
        }
    }

    panic!("Input does not contain a solution !")
}


fn part02(file_name: &str) -> i32 {
    let mut entries: Vec<i32> = read_to_string(file_name).unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    entries.sort();

    for i in 0..entries.len() {
        for k in i..entries.len() {
            for j in k..entries.len() {
                let first = entries[i];
                let second = entries[k];
                let third = entries[j];
                let sum = first + second + third;

                if sum == SUM_OBJECTIVE {
                    return first * second * third;
                }

                if sum > SUM_OBJECTIVE {
                    break;
                }
            }
        }
    }

    panic!("Input does not contain a solution !")
}