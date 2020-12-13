use crate::{get_result, get_result_i64};

// https://adventofcode.com/2020/day/13
// https://www.reddit.com/r/rust/comments/kc5phc/advent_of_code_2020_day_13/

const INPUT_FILENAME: &str = "inputs/input13";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result_i64(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    let (first_line, second_line) = input.split_once('\n').unwrap();
    let timestamp = first_line.parse::<usize>().unwrap();
    let id = second_line
        .split(',')
        .filter_map(|id| id.parse::<usize>().ok())
        .min_by_key(|id| id - (timestamp % id))
        .unwrap();

    id * (id - (timestamp % id))
}

fn part02(input: String) -> i64 {
    let (_, second_line) = input.split_once('\n').unwrap();
    let values = second_line
        .split(',')
        .enumerate()
        .filter_map(|(idx, bus)| bus.parse::<i64>().ok()
            .map(|bus| (bus - (idx % bus as usize) as i64, bus)))
        .collect::<Vec<(i64, i64)>>();
    let product: i64 = values.iter().fold(1, |acc, (_, bus)| acc * bus);
    values.iter()
        .map(|(idx, bus)| (idx, bus, product / bus))
        .map(|(&idx, &bus, partial)| (idx * euclid_inverse(partial, bus) * partial) % product)
        .sum::<i64>() % product
}

// https://www.freecodecamp.org/news/how-to-implement-the-chinese-remainder-theorem-in-java-db88a3f1ffe0/
fn euclid_inverse(mut a: i64, mut b: i64) -> i64 {
    let m = b;
    let mut t;
    let mut q;
    let mut x = 0;
    let mut y = 1;

    if b == 1 {
        return 0;
    }

    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x;
        x = y - q * x;
        y = t;
    }

    if y < 0 {
        y += m;
    }

    y
}

