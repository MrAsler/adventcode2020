#![allow(dead_code)]
#![feature(iterator_fold_self)]
#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;
use std::time::SystemTime;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;


fn main() {
    day14::solve();
}

pub fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

pub fn get_result<F>(part: u8, func: F, file_name: &str)
    where F: Fn(String) -> usize {
    let start = SystemTime::now();
    let result = func(read_input(file_name));
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Part {}: {} (time: {:?})", part, result, duration);
}

pub fn get_result_i32<F>(part: u8, func: F, file_name: &str)
    where F: Fn(String) -> i32 {
    let start = SystemTime::now();
    let result = func(read_input(file_name));
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Part {}: {} (time: {:?})", part, result, duration);
}

pub fn get_result_i64<F>(part: u8, func: F, file_name: &str)
    where F: Fn(String) -> i64 {
    let start = SystemTime::now();
    let result = func(read_input(file_name));
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("Part {}: {} (time: {:?})", part, result, duration);
}