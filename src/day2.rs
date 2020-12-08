use parse_display::{Display as PDisplay, FromStr as PFromStr};
use crate::get_result;

// https://www.reddit.com/r/rust/comments/k554uk/advent_of_code_2020_day_2/

const INPUT_FILENAME: &str = "inputs/input2";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .filter(|e| is_valid_part_1(e))
        .count()
}


fn part02(input: String) -> usize {
    input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .filter(|e| is_valid_part_2(e))
        .count()
}


fn is_valid_part_1(entry: &Entry) -> bool {
    let count: usize = entry.password.clone().matches(entry.letter).count();
    return count >= entry.first && count <= entry.second;
}

fn is_valid_part_2(entry: &Entry) -> bool {
    let vec: Vec<char> = entry.password.chars().collect();
    let first_letter = vec[entry.first - 1];
    let second_letter = vec[entry.second - 1];

    (first_letter == entry.letter) ^ (second_letter == entry.letter)
}

#[derive(PDisplay, PFromStr, Debug)]
#[display("{first}-{second} {letter}: {password}")]
struct Entry {
    first: usize,
    second: usize,
    letter: char,
    password: String
}

/* Old code, before reddit
use std::str::FromStr;
use std::error::Error;

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let mut min_max = split.nth(0).unwrap().split('-');
        let letter = split.nth(0).unwrap().trim_matches(':').chars().next().unwrap();

        Ok(Entry {
            first: min_max.nth(0).unwrap().parse::<usize>().unwrap(),
            second: min_max.nth(0).unwrap().parse::<usize>().unwrap(),
            letter,
            password: split.nth(0).unwrap().to_string()
        })
    }
}
*/