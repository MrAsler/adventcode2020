use regex::Regex;
use crate::read_input;


// https://www.reddit.com/r/rust/comments/k6fgrs/advent_of_code_2020_day_4/

const INPUT_FILENAME: &str = "inputs/input4";

pub fn solve() {
    println!("Part 1: {}", solution(read_input(INPUT_FILENAME), Validator::PartOne));
    println!("Part 2: {}", solution(read_input(INPUT_FILENAME), Validator::PartTwo));
}

fn solution(input: String, validator: Validator) -> usize {
    let mut result = 0;
    let mut passport = Passport::new();
    for line in input.lines() {
        for entry in line.split_ascii_whitespace() {
            let mut split = entry.split(':');
            let token = split.next().unwrap();
            let value = split.next().unwrap();
            match token {
                "byr" => passport.byr = Some(value.parse::<usize>().unwrap()),
                "iyr" => passport.iyr = Some(value.parse::<usize>().unwrap()),
                "eyr" => passport.eyr = Some(value.parse::<usize>().unwrap()),
                "hgt" => passport.hgt = Some(value.to_string()),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.parse::<usize>().unwrap()),
                _ => panic!()
            }
        }
        if line.is_empty() {
            if passport.is_valid(&validator) {
                result += 1;
            }
            passport = Passport::new();
        }
    }
    if passport.is_valid(&validator) {
        result += 1;
    }

    result
}

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl Passport {
    fn is_valid(&self, validator: &Validator) -> bool {
        match validator {
            Validator::PartOne => self.is_valid_pt1(),
            Validator::PartTwo => self.is_valid_pt2()
        }
    }

    fn is_valid_pt1(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() &&
            self.ecl.is_some() && self.pid.is_some()
    }

    fn is_valid_pt2(&self) -> bool {
        self.byr.is_some() && self.byr.map(|v| v >= 1920 && v <= 2002).unwrap() &&
            self.iyr.is_some() && self.iyr.map(|v| v >= 2010 && v <= 2020).unwrap() &&
            self.eyr.is_some() && self.eyr.map(|v| v >= 2020 && v <= 2030).unwrap() &&
            self.hgt.is_some() && self.hgt.as_ref().map(|v| validate_height(&v)).unwrap() &&
            self.hcl.is_some() && self.hcl.as_ref().map(|v| validate_hair_color(&v)).unwrap() &&
            self.ecl.is_some() && self.ecl.as_ref().map(|v| validate_eye_color(&v)).unwrap() &&
            self.pid.is_some() && self.pid.as_ref().map(|v| v.len() == 9 && v.parse::<u32>().is_ok()).unwrap()
    }

    fn new() -> Passport {
        Passport { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None }
    }
}

fn validate_height(height: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([0-9]*)(cm|in)$").unwrap();
    }

    let captures = match RE.captures(&height) {
        Some(res) => res,
        None => return false,
    };
    let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let metric = captures.get(2).unwrap().as_str();

    match metric {
        "cm" => size >= 150 && size <= 193,
        "in" => size >= 59 && size <= 76,
        _ => panic!()
    }
}

fn validate_hair_color(hair: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#([a-fA-F0-9]{6})$").unwrap();
    }
    RE.is_match(&hair)
}

fn validate_eye_color(eye: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    RE.is_match(&eye)
}

enum Validator {
    PartOne,
    PartTwo,
}