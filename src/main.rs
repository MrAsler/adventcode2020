#![allow(dead_code)]
#![feature(iterator_fold_self)]

#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    day7::solve()
}

pub fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}
