use crate::{get_result, get_result_i64};
use std::collections::HashMap;

// https://adventofcode.com/2020/day/10
// https://www.reddit.com/r/rust/comments/ka9nre/advent_of_code_2020_day_10/

const INPUT_FILENAME: &str = "inputs/input10";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result_i64(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    let mut differences: HashMap<usize, usize> = HashMap::new();
    let sorted_values = get_sorted_input(input);
    sorted_values.windows(2)
        .for_each(|x| *differences.entry(x[1] - x[0]).or_insert(0) += 1);
    *differences.entry(sorted_values[0]).or_insert(0) += 1; // Add base
    *differences.entry(3).or_insert(0) += 1; // Add built in diff

    differences[&1] * differences[&3]
}

// Key insight for part 2: num_paths(x) = num_paths(x-1) + num_paths(x-2) + num_paths(x-3)
// Aka the tribonacci
fn part02(input: String) -> i64 {
    let mut sorted_input = get_sorted_input(input);
    sorted_input.insert(0, 0);
    let mut dynamic: Vec<i64> = Vec::new();
    dynamic.push(1);

    for i in 1..sorted_input.len() {
        let sum =
            get_value(i, (i as i64 - 1) as i64, &sorted_input, &dynamic) +
            get_value(i, (i as i64 - 2) as i64, &sorted_input, &dynamic) +
            get_value(i, (i as i64 - 3) as i64, &sorted_input, &dynamic);
        dynamic.push(sum);
    }

    *dynamic.last().unwrap()
}

fn get_value(current_idx: usize, target_idx: i64, sorted_input: &Vec<usize>, dynamic: &Vec<i64>) -> i64 {
    let current = sorted_input[current_idx as usize];
    let previous = match sorted_input.get(target_idx as usize) {
        Some(x) => x,
        None => return 0
    };

    if current - previous > 3 {
        return 0;
    }

    return dynamic[target_idx as usize];
}

fn get_sorted_input(input: String) -> Vec<usize> {
    let mut sorted_values = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    sorted_values.sort();
    sorted_values
}