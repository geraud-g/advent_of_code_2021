use std::cmp::Ordering;
use std::collections::HashMap;

use advent_of_code_2021::utils::inputs::get_file;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}


impl Coord {
    fn from_str(string: &str) -> Self {
        let split = string.split(',');
        let mut casted = split.map(|s| i32::from_str_radix(s, 10));
        let x = casted.next().unwrap().unwrap();
        let y = casted.next().unwrap().unwrap();
        Coord { y, x }
    }
}

type Line = (Coord, Coord);


pub fn day_05() {
    let lines = get_lines();

    let no_diagonals: Vec<_> = lines.iter().filter(|l| !is_diagonal(l)).cloned().collect();
    let solution_a = get_nbr_overlapping_points(&no_diagonals);
    println!("Solution for Day 5, part A is: {}", solution_a);

    let solution_b = get_nbr_overlapping_points(&lines);
    println!("Solution for Day 5, part B is: {}", solution_b);
}


fn get_lines() -> Vec<Line> {
    get_file("./src/day_05/input.txt")
        .lines()
        .map(|s| line_from_string(s))
        .collect()
}

fn is_diagonal(line: &Line) -> bool {
    line.0.y != line.1.y && line.0.x != line.1.x
}

fn line_from_string(string: &str) -> Line {
    let mut split = string.split(" -> ");
    let left = Coord::from_str(split.next().unwrap());
    let right = Coord::from_str(split.next().unwrap());
    (left, right)
}


fn get_nbr_overlapping_points(lines: &[Line]) -> usize {
    let mut ocean_floor = HashMap::new();
    for line in lines {
        draw_line(&mut ocean_floor, line)
    }
    ocean_floor.values().filter(|&val| *val > 1).count()
}


fn draw_line(ocean_floor: &mut HashMap<Coord, usize>, line: &Line) {
    let (coord_a, coord_b) = line;
    let delta_x = get_delta(coord_a.x, coord_b.x);
    let delta_y = get_delta(coord_a.y, coord_b.y);

    *ocean_floor.entry(coord_a.clone()).or_insert(0) += 1;
    let mut current_point = *coord_a;

    while current_point != *coord_b {
        current_point = Coord { y: current_point.y + delta_y, x: current_point.x + delta_x };
        *ocean_floor.entry(current_point).or_insert(0) += 1;
    }
}

fn get_delta(val_a: i32, val_b: i32) -> i32 {
    match val_a.cmp(&val_b) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}
