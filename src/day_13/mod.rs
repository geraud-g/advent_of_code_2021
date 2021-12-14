use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use advent_of_code_2021::utils::inputs::{get_file, LINE_ENDING};
use advent_of_code_2021::utils::point::Point;


lazy_static! {
    static ref REGEX_FOLD_Y: Regex = Regex::new(r"fold along y=(\d+)").unwrap();
    static ref REGEX_FOLD_X: Regex = Regex::new(r"fold along x=(\d+)").unwrap();
}


pub fn day_13() {
    let (points, instructions) = get_input();

    let solution_a = fold_paper(&points, &instructions[..1]);
    println!("Solution for Day 13, part A is: {}", solution_a.len());

    let solution_b = fold_paper(&points, &instructions);
    println!("Solution for Day 13, part B is:");
    print(&solution_b);
}


fn get_input() -> (HashSet<Point>, Vec<Fold>) {
    let split_val = &format!("{}{}", LINE_ENDING, LINE_ENDING);
    let data = get_file("./src/day_13/input.txt");
    let mut data = data.split(split_val);
    let points = get_points(&data.next().unwrap());
    let fold_instructions = get_instructions(&data.next().unwrap());
    (points, fold_instructions)
}


fn get_points(string: &str) -> HashSet<Point> {
    let to_usize = |l: &str| usize::from_str_radix(l, 10).unwrap();
    string.lines()
        .map(|l| {
            let mut line = l.split(',');
            let x = to_usize(&line.next().unwrap());
            let y = to_usize(&line.next().unwrap());
            Point::from_usize(y, x)
        })
        .collect()
}


fn get_instructions(string: &str) -> Vec<Fold> {
    string.lines()
        .map(|l| {
            if let Some(result) = REGEX_FOLD_Y.captures(l) {
                Fold::Y(result[1].parse().unwrap())
            } else if let Some(result) = REGEX_FOLD_X.captures(l) {
                Fold::X(result[1].parse().unwrap())
            } else {
                panic!("Wrong value: {}", l)
            }
        }).collect()
}


#[derive(Debug)]
enum Fold {
    Y(usize),
    X(usize),
}


fn fold_paper(points: &HashSet<Point>, instructions: &[Fold]) -> HashSet<Point> {
    let mut paper = points.clone();

    for instruction in instructions {
        paper = match instruction {
            Fold::Y(axis) => fold_y_axis(&paper, *axis),
            Fold::X(axis) => fold_x_axis(&paper, *axis)
        };
    }
    paper
}


fn fold_y_axis(points: &HashSet<Point>, axis: usize) -> HashSet<Point> {
    let mut new_paper = HashSet::new();
    for point in points {
        if point.y <= axis {
            new_paper.insert(*point);
        } else {
            let delta = point.y - axis;
            new_paper.insert(Point::from_usize(axis - delta, point.x));
        }
    }
    new_paper
}


fn fold_x_axis(points: &HashSet<Point>, axis: usize) -> HashSet<Point> {
    let mut new_paper = HashSet::new();

    for point in points {
        if point.x <= axis {
            new_paper.insert(*point);
        } else {
            let delta = point.x - axis;
            new_paper.insert(Point::from_usize(point.y, axis - delta));
        }
    }
    new_paper
}


fn print(points: &HashSet<Point>) {
    let max_y = points.iter().map(|p| p.y).max().unwrap() + 1;
    let max_x = points.iter().map(|p| p.x).max().unwrap() + 1;
    for y in 0..max_y {
        let mut line = String::new();
        for x in 0..max_x {
            let val = if points.contains(&Point::from_usize(y, x)) { "█" } else { " " };
            line.push_str(val);
        }
        println!("{}", line)
    }
}
