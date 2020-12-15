use crate::get_result;

// https://adventofcode.com/2020/day/15
// https://www.reddit.com/r/rust/comments/kdh7rr/advent_of_code_2020_day_15/

const INPUT_FILENAME: &str = "inputs/input15";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    let input: Vec<_> = input.trim_end().split(',').map(|n| n.parse().unwrap()).collect();
    play(&input, 2020)
}

fn part02(input: String) -> usize {
    let input: Vec<_> = input.trim_end().split(',').map(|n| n.parse().unwrap()).collect();
    play(&input, 30000000)
}

fn play(input: &[usize], number_rounds: usize) -> usize {
    let mut vec: Vec<usize> = vec![0; number_rounds + 1];
    for i in 0..input.len() {
        vec[input[i]] = i + 1;
    }
    vec[*input.last().unwrap()] = 0;

    let mut last_word = *input.last().unwrap();
    for turn in input.len() + 1..=number_rounds {
        let mut last_spoken = vec[last_word];
        if last_spoken != 0 {
            last_spoken = turn - 1 -  last_spoken;
        }
        vec[last_word] = turn - 1;
        last_word = last_spoken;
    }

    last_word
}
