use crate::get_result_i64;

// https://adventofcode.com/2020/day/9
// https://www.reddit.com/r/rust/comments/k9m4ml/advent_of_code_2020_day_9/
// Array windows makes this pretty fun (is currently unstable)

const INPUT_FILENAME: &str = "inputs/input09";

pub fn solve() {
    get_result_i64(1, part01, INPUT_FILENAME);
    get_result_i64(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> i64 {
    let preamble = 25;
    let numbers = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for i in preamble + 1..numbers.len() {
        if !find_sum(&numbers, i, preamble) {
            return numbers[i]
        }
    }
    panic!()
}

fn part02(input: String) -> i64 {
    let invalid_number = part01(input.clone());
    let mut numbers = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let invalid_number_idx = numbers.iter().enumerate().find(|(_, target)| **target == invalid_number).unwrap().0;
    let sub_vector = &mut numbers[0..invalid_number_idx];

    for smallest_idx in 0..sub_vector.len() {
        let mut count = sub_vector[smallest_idx];
        let mut largest_idx = smallest_idx + 1;
        let mut min: i64 = sub_vector[smallest_idx];
        let mut max: i64 = sub_vector[smallest_idx];
        while count < invalid_number {
            count += sub_vector[largest_idx];
            min = min.min(sub_vector[largest_idx]);
            max = max.max(sub_vector[largest_idx]);
            if count == invalid_number {
                return (min + max) as i64;
            }
            largest_idx += 1;
        }
    }
    panic!()
}

fn find_sum(numbers: &Vec<i64>, current: usize, preamble: usize) -> bool {
    let sub_set = &numbers[current - 1 - preamble..current];
    let target = numbers[current];
    for value in sub_set {
        if sub_set.contains(&(target - value)) {
            return true
        }
    }
    false
}

