use std::collections::HashMap;
use crate::get_result;

// https://adventofcode.com/2020/day/7
// https://www.reddit.com/r/rust/comments/k8ador/advent_of_code_2020_day_7/

const INPUT_FILENAME: &str = "inputs/input07";
const TARGET: &str = "shiny gold";

pub fn solve() {
    get_result(1, part01, INPUT_FILENAME);
    get_result(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> usize {
    let bags = parse_bags(input);
    get_number_target(bags, TARGET)
}

fn part02(input: String) -> usize {
    let bags = parse_bags(input);
    get_bags_inside_target(&bags[TARGET], &bags) - 1
}

fn get_number_target(bags: HashMap<String, Bag>, target: &str) -> usize {
    bags.values().filter(|bag| can_contain_bag(bag, &bags, &target)).count()
}

fn get_bags_inside_target(bag: &Bag, bags: &HashMap<String, Bag>) -> usize {
    bag.contents.iter().fold(1, |acc, entry| acc + (get_bags_inside_target(&bags[entry.0], bags) * entry.1))
}


fn parse_bags(input: String) -> HashMap<String, Bag> {
    input.lines().map(|line| parse_line(line)).map(|bag| (bag.name.clone(), bag)).collect()
}

fn can_contain_bag(bag: &Bag, all_bags: &HashMap<String, Bag>, target: &str) -> bool {
    for contained_bag in bag.contents.keys() {
        if contained_bag == target {
            return true;
        } else {
            if can_contain_bag(all_bags.get(contained_bag).unwrap(), all_bags, target) {
                return true
            }
        }
    }
    return false
}

fn parse_line(line: &str) -> Bag {
    let split: Vec<&str> = line.split("bags contain").map(|str| str.trim()).collect();
    let bag_identifier = split[0];
    let contents: HashMap<String, usize> = split[1].split(',')
        .map(|str| str.trim().trim_end_matches('.').trim_end_matches(" bags")).filter(|str| !str.starts_with("no"))
        .map(|str| get_number_bags(str)).collect();
    Bag {
        name: bag_identifier.to_string(),
        contents
    }
}

fn get_number_bags(entry: &str) -> (String, usize) {
    let split: Vec<String> = entry.split_ascii_whitespace().map(|s| s.to_string()).collect();
    let number = split[0].parse::<usize>().unwrap();
    let bag_name = format!("{} {}", split[1], split[2]);
    (bag_name, number)
}

#[derive(Debug)]
struct Bag {
    name: String,
    contents: HashMap<String, usize>
}