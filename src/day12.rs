use crate::get_result_i32;

// https://adventofcode.com/2020/day/12
// https://www.reddit.com/r/rust/comments/kbk3r7/advent_of_code_2020_day_12/

const INPUT_FILENAME: &str = "inputs/input12";

pub fn solve() {
    get_result_i32(1, part01, INPUT_FILENAME);
    get_result_i32(2, part02, INPUT_FILENAME);
}

fn part01(input: String) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut angle: i32 = 0;
    input.lines()
        .map(|input| input.split_at(1))
        .map(|(action, value)| (action, value.parse::<i32>().unwrap()))
        .for_each(|(action, value)| {
            match action {
                "F" => move_to_direction_p1(angle_to_direction_p1(angle), value, &mut position),
                "L" => angle = ((angle + value) % 360).abs(),
                "R" => angle = ((360 - value + angle) % 360).abs(),
                action => move_to_direction_p1(action, value, &mut position)
            }
        });
    position.0.abs() + position.1.abs()
}

fn move_to_direction_p1(direction: &str, value: i32, position: &mut (i32, i32)) {
    match direction {
        "N" => position.1 += value,
        "S" => position.1 -= value,
        "E" => position.0 += value,
        "W" => position.0 -= value,
        _ => unreachable!()
    }
}

fn angle_to_direction_p1(angle: i32) -> &'static str {
    match angle {
        0 => "E",
        90 => "N",
        180 => "W",
        270 => "S",
        _ => unreachable!()
    }
}


fn part02(input: String) -> i32 {
    let mut waypoint: (i32, i32) = (10, 1);
    let mut ship_pos: (i32, i32) = (0, 0);
    input.lines()
        .map(|input| input.split_at(1))
        .map(|(action, value)| (action, value.parse::<i32>().unwrap()))
        .for_each(|(action, value)| {
            match action {
                "F" => ship_pos = (ship_pos.0 + value * waypoint.0, ship_pos.1 + value * waypoint.1),
                "L" => rotate_waypoint(value % 360, &mut waypoint),
                "R" => rotate_waypoint((360 - value) % 360, &mut waypoint),
                action => move_waypoint(action,  &mut waypoint, value)
            }
        });
    ship_pos.0.abs() + ship_pos.1.abs()
}

fn move_waypoint(direction: &str, waypoint: &mut (i32, i32), value: i32) {
    match direction {
        "N" => *waypoint = (waypoint.0, waypoint.1 + value),
        "S" => *waypoint = (waypoint.0, waypoint.1 - value),
        "E" => *waypoint = (waypoint.0 + value, waypoint.1),
        "W" => *waypoint = (waypoint.0 - value, waypoint.1),
        _ => unreachable!()
    }
}

fn rotate_waypoint(angle: i32, waypoint: &mut (i32, i32)) {
    match angle {
        0 => (),
        90 => *waypoint = (-waypoint.1, waypoint.0),
        180 => *waypoint = (-waypoint.0, -waypoint.1),
        270 => *waypoint = (waypoint.1, -waypoint.0),
        _ => unreachable!()
    }
}
