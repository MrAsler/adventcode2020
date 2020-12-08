use std::collections::HashSet;
use crate::read_input;

// https://www.reddit.com/r/rust/comments/k7nq0h/advent_of_code_2020_day_6/

const INPUT_FILENAME: &str = "inputs/input6";

pub fn solve() {
    println!("Part 1: {}", part01(read_input(INPUT_FILENAME)));
    println!("Part 2: {}", part02(read_input(INPUT_FILENAME)));
}

fn part01(input: String) -> usize {
    input.split("\n\n")
        .map(|answers| answers.trim().replace('\n', "").chars().collect::<HashSet<char>>())
        .fold(0, |acc, set| acc + set.len())
}

fn part02(input: String) -> usize {
    input.split("\n\n")
        .map(|group| group.trim().split('\n').
            map(|answer| answer.chars().collect::<HashSet<char>>()).
            fold_first(|acc, set| acc.intersection(&set).map(|c| *c).collect()))
        .fold(0, |acc, set| acc + set.unwrap().len())
}