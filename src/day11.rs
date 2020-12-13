use crate::get_result;
use itertools::Itertools;

// https://adventofcode.com/2020/day/11
// https://www.reddit.com/r/rust/comments/kb3umi/advent_of_code_2020_day_11/

const INPUT_FILENAME: &str = "inputs/input11";

enum Part {
    ONE,
    TWO,
}

pub fn solve() {
    get_result(1, part01_optimized, INPUT_FILENAME);
    get_result(2, part02, INPUT_FILENAME);
}


fn part01_optimized(input: String) -> usize {
    let mut layout = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    loop {
        let changes: Vec<(usize, usize, char)> = do_round_optimized(&layout, Part::ONE);
        if changes.is_empty() {
            return layout.iter().map(|vec| vec.iter().filter(|c| **c == '#').count()).sum();
        }
        changes.iter().for_each(|(y, x, seat)| layout[*y][*x] = *seat);
    }
}

fn part02(input: String) -> usize {
    let mut layout = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    loop {
        let changes: Vec<(usize, usize, char)> = do_round_optimized(&layout, Part::TWO);
        if changes.is_empty() {
            return layout.iter().map(|vec| vec.iter().filter(|c| **c == '#').count()).sum();
        }
        changes.iter().for_each(|(y, x, seat)| layout[*y][*x] = *seat);
    }
}

fn do_round_optimized(layout: &Vec<Vec<char>>, part: Part) -> Vec<(usize, usize, char)> {
    let mut result: Vec<(usize, usize, char)> = Vec::new();
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            let current_location = layout[y][x];
            if current_location == '.' {
                continue;
            };
            match part {
                Part::ONE => {
                    match get_occupied_seats_optimized(x, y, &layout) {
                        0 if current_location == 'L' => result.push((y, x, '#')),
                        val if val >= 4 && current_location == '#' => result.push((y, x, 'L')),
                        _ => ()
                    }
                }
                Part::TWO => {
                    match get_occupied_seats_part2(x, y, &layout) {
                        0 if current_location == 'L' => result.push((y, x, '#')),
                        val if val >= 5 && current_location == '#' => result.push((y, x, 'L')),
                        _ => ()
                    }
                }
            }
        }
    }
    result
}

fn get_occupied_seats_optimized(x: usize, y: usize, layout: &Vec<Vec<char>>) -> usize {
    (-1..=1i32).cartesian_product(-1..=1i32)
        .filter(|(dx, dy)| (dx, dy) != (&0, &0))
        .map(|(dx, dy)| (dx + (x as i32), dy + (y as i32)))
        .map(|(dx, dy)| layout.get(dy as usize).and_then(|row| row.get(dx as usize)))
        .filter(|val| match val {
            Some(seat) => **seat == '#',
            _ => false
        }).count()
}


fn get_occupied_seats_part2(x: usize, y: usize, layout: &Vec<Vec<char>>) -> usize {
    let directions = (-1..=1i32).cartesian_product(-1..=1i32)
        .filter(|(dx, dy)| (dx, dy) != (&0, &0))
        .collect::<Vec<(i32, i32)>>();
    let mut occupied = 0;
    for (direction_x, direction_y) in directions {
        let mut current_x = x as i32 + direction_x;
        let mut current_y = y as i32 + direction_y;
        loop {
            match layout.get(current_y as usize).and_then(|row| row.get(current_x as usize)) {
                Some(seat) if *seat == '#' => {
                    occupied += 1;
                    break;
                }
                Some(seat) if *seat == 'L' => break,
                Some(_) => (),
                None => {
                    break
                }
            }
            current_x += direction_x;
            current_y += direction_y;
        }
    }
    occupied
}


/*

use std::collections::HashMap;

fn part01_v1(input: String) -> usize {
    let mut layout = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut changes = 0;
    let mut round = 0;
    loop {
        /*
        println!("Round {}:", round);
        layout.iter().for_each(|x| {
            x.iter().for_each(|c| print!("{}", c));
            println!();
        });
        println!();
        println!();
        */
        (layout, changes) = do_round_v1(layout.clone());
        if changes == 0 {
            return layout.iter().map(|vec| vec.iter().filter(|c| **c == '#').count()).sum()
        }

        round += 1;
    }
}

fn do_round_v1(layout: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut result = layout.clone();
    let adjacent_seats_perm = (-1..=1).cartesian_product(-1..=1).collect::<Vec<(i32, i32)>>();
    let mut changes = 0;
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
        //    println!("Busy seats for {:?}: {:?}",  (x, y), get_occupied_seats(x, y, &layout, &adjacent_seats_perm));
            match get_occupied_seats_v1(x, y, &layout, &adjacent_seats_perm) {
                Some(0) => {
                    if layout[y][x] == 'L' {
                        result[y][x] = '#';
                       // println!("BOOP1 AT {:?}", (x, y));
                        changes += 1;
                    }
                },
                Some(z) if z >= 4 => {
                    if layout[y][x] == '#' {
                        result[y][x] = 'L';
              //          println!("BOOP2 AT {:?}", (x, y));
                        changes += 1;
                    }
                },
                _ => ()
            }
        }
    }
    (result, changes)
}

fn get_occupied_seats_v1(x: usize, y: usize, layout: &Vec<Vec<char>>, adjacent_seats_perm: &Vec<(i32, i32)>) -> Option<usize> {
    let max_size_x = layout[0].len();
    let legit_adjacent_seats = adjacent_seats_perm.clone()
        .iter_mut()
        .map(|(x_d, y_d)| ((x as i32 ) + *x_d, (y as i32) + *y_d))
        .filter(|(mod_x, mod_y)| *mod_x >= 0 && *mod_x < max_size_x as i32 && *mod_y >= 0 && *mod_y < layout.len() as i32)
        .filter(|(mod_x, mod_y)| (*mod_x, *mod_y) != (x as i32, y as i32))
        .map((|(x, y)| (x as usize, y as usize)))
        .collect::<Vec<(usize, usize)>>();

    if layout[y][x] == '.' {
        return None
    };

    Some(legit_adjacent_seats.iter().filter(|(x, y)| layout[*y][*x] == '#').count())
}







fn part01_hashmaps(input: String) -> usize {
    let mut layout = input.lines()
        .enumerate()
        .map(|(x, line)| (x as i32, line.chars().enumerate().map(|(y, c)| (y as i32, c)).collect::<HashMap<i32, char>>()))
        .collect::<HashMap<i32, HashMap<i32, char>>>();
    //println!("{:?}", layout);
    let mut changes = 0;
    let mut round = 0;
    loop {
/*
        println!("Round {}:", round);

        for x in 0..layout.len() {
            for y in 0..layout[&(x as i32)].len() {
                print!("{}", layout[&(x as i32)][&(y as i32)]);
            }
            println!();
        }
        println!();
        println!();
*/

        (layout, changes) = do_round_hashmaps(layout.clone());
        if changes == 0 {
            return layout.iter().map(|(x, y_row)| y_row.values().filter(|c| **c == '#').count()).sum()
        }

        round += 1;
    }
}




fn do_round_hashmaps(layout: HashMap<i32, HashMap<i32, char>>) -> (HashMap<i32, HashMap<i32, char>>, usize) {
    let mut result = layout.clone();
    let adjacent_seats_perm = (-1..=1).cartesian_product(-1..=1).collect::<Vec<(i32, i32)>>();
    let mut changes = 0;
    for x in 0..layout.len() {
        for y in 0..layout[&(x as i32)].len() {
        //    println!("Busy seats for {:?}: {:?}",  (x, y), get_occupied_seats_v2(x as i32, y as i32, &layout, &adjacent_seats_perm));
            match get_occupied_seats_hashmaps(x as i32, y as i32, &layout, &adjacent_seats_perm) {
                Some(0) => {
                    if layout[&(x as i32)][&(y as i32)] == 'L' {
                        result.get_mut(&(x as i32)).unwrap().insert(y as i32, '#');
           //             println!("BOOP1 AT {:?}", (x, y));
                        changes += 1;
                    }
                },
                Some(z) if z >= 4 => {
                    if layout[&(x as i32)][&(y as i32)] == '#' {
                        result.get_mut(&(x as i32)).unwrap().insert(y as i32, 'L');
             //           println!("BOOP2 AT {:?}", (x, y));
                        changes += 1;
                    }
                },
                _ => ()
            }
        }
    }
    (result, changes)
}


fn get_occupied_seats_hashmaps(x: i32, y: i32, layout: &HashMap<i32, HashMap<i32, char>>, adjacent_seats_perm: &Vec<(i32, i32)>) -> Option<usize> {
    if layout[&x][&y] == '.' {
        return None
    };

    Some(adjacent_seats_perm.iter()
        .map(|(x_mod, y_mod)| (*x_mod + x, *y_mod + y))
        .filter(|(mod_x, mod_y)| (*mod_x, *mod_y) != (x as i32, y as i32))
        .filter(|(x_mod, y_mod)| {
            if layout.contains_key(x_mod) && layout[x_mod].contains_key(y_mod) {
                layout[x_mod][y_mod] == '#'
            } else {
                false
            }
        }).count())
}

 */

