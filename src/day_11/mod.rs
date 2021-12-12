use ansi_term::Color;
use itertools::iproduct;

use advent_of_code_2021::utils::inputs::get_file;
use advent_of_code_2021::utils::point::Point;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;


pub fn day_11() {
    let mut octopuses = get_input();

    let solution_a = count_flashes(&mut octopuses, 100);
    println!("Solution for Day 11, part A is: {}", solution_a);

    let mut octopuses = get_input();
    let solution_b = get_sync_step(&mut octopuses);
    println!("Solution for Day 11, part B is: {}", solution_b);
}


fn get_input() -> Vec<Vec<u32>> {
    get_file("./src/day_11/input.txt").lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}


fn count_flashes(octopuses: &mut [Vec<u32>], steps: usize) -> u32 {
    let mut flashes = 0;

    for _ in 0..steps {
        inc_counter(octopuses);
        eval_flash(octopuses);

        for (y, x) in iproduct!(0..HEIGHT, 0..WIDTH) {
            if octopuses[y][x] > 9 {
                flashes += 1;
                octopuses[y][x] = 0;
            }
        }
    }
    flashes
}

fn get_sync_step(octopuses: &mut [Vec<u32>]) -> u32 {
    let mut steps = 0;

    loop {
        inc_counter(octopuses);
        eval_flash(octopuses);
        for (y, x) in iproduct!(0..HEIGHT, 0..WIDTH) {
            if octopuses[y][x] > 9 {
                octopuses[y][x] = 0;
            }
        }
        steps += 1;
        if octopuses.iter().flat_map(|x| x).all(|l| l == &0) {
            return steps;
        }
    }
}


fn inc_counter(octopuses: &mut [Vec<u32>]) {
    for (y, x) in iproduct!(0..HEIGHT, 0..WIDTH) {
        octopuses[y][x] += 1
    }
}


fn eval_flash(octopuses: &mut [Vec<u32>]) {
    let mut flashed = vec![];

    loop {
        let last_flashed = &flashed.to_vec();
        for (y, x) in iproduct!(0..HEIGHT, 0..WIDTH) {
            let point = Point::from_usize(y, x);
            if octopuses[y][x] > 9 && !flashed.contains(&point) {
                apply_flash(octopuses, &point);
                flashed.push(point)
            }
        }
        if flashed.len() == last_flashed.len() {
            return;
        }
    }
}


fn apply_flash(octopuses: &mut [Vec<u32>], point: &Point) {
    for neighbour in point.get_neighbours_with_diag(HEIGHT, WIDTH) {
        octopuses[neighbour.y][neighbour.x] += 1;
    }
}
