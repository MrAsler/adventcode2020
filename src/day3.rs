use std::fs::read_to_string;
use std::collections::HashMap;

// https://www.reddit.com/r/rust/comments/k5s4k9/advent_of_code_2020_day_3/

const INPUT_FILENAME: &str = "inputs/input3";


pub fn solve() {
    println!("Part 1: {}", part01(INPUT_FILENAME));
    println!("Part 2: {}", part02(INPUT_FILENAME));
}

struct Map {
    map: HashMap<usize, HashMap<usize, bool>>,
    x_dimension: usize,
    y_dimension: usize
}

struct SlopeType {
    right: usize,
    down: usize
}

impl Map {

    fn new(file_name: &str) -> Map {
        let mut map: HashMap<usize, HashMap<usize, bool>> = HashMap::new();
        for (idx_y, line) in read_to_string(file_name).unwrap().lines().enumerate() {
            &map.insert(idx_y, HashMap::new());
            for (idx_x, char) in line.char_indices() {
                &map.get_mut(&idx_y).unwrap().insert(idx_x, char == '#');
            }
        }

        let x_dimension = map[&0].len();
        let y_dimension = map.len();

        Map {
            map,
            x_dimension,
            y_dimension
        }
    }

    fn position_has_tree(&self, x: usize, y: usize) -> bool {
        self.map[&y][&(x % self.x_dimension)]
    }

    fn slide_down_get_trees(&self, slope: &SlopeType) -> usize {
        let mut trees = 0;
        let mut x = 0;
        let mut y = 0;

        while y < self.y_dimension {
            if self.position_has_tree(x, y) {
                trees += 1;
            }
            x += slope.right;
            y += slope.down;
        }

        trees
    }
}

fn part01(file_name: &str) -> usize {
    Map::new(file_name).slide_down_get_trees(&SlopeType{right: 3, down: 1})
}

fn part02(file_name: &str) -> usize {
    let map = Map::new(file_name);
    map.slide_down_get_trees(&SlopeType{right: 1, down: 1}) *
        map.slide_down_get_trees(&SlopeType{right: 3, down: 1}) *
        map.slide_down_get_trees(&SlopeType{right: 5, down: 1}) *
        map.slide_down_get_trees(&SlopeType{right: 7, down: 1}) *
        map.slide_down_get_trees(&SlopeType{right: 1, down: 2})
}