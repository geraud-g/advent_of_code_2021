use itertools::Itertools;

use advent_of_code_2021::utils::inputs::get_file;

pub fn day_02() {
    let instructions = get_input();

    let solution_a = follow_course(&instructions);
    println!("Solution for Day 2, part A is: {}", solution_a);

    let solution_b = follow_course_part_b(&instructions);
    println!("Solution for Day 2, part B is: {}", solution_b);
}


fn get_input() -> Vec<(String, i32)> {
    get_file("./src/day_02/input.txt")
        .lines()
        .map(|s| s.split(' ').collect_tuple().unwrap())
        .map(|split: (&str, &str)| (split.0.to_string(), split.1.parse::<i32>().unwrap()))
        .collect()
}


fn follow_course(instructions: &[(String, i32)]) -> i32 {
    let mut h_position = 0;
    let mut depth = 0;

    for (instruction, value) in instructions {
        match &instruction[..] {
            "forward" => h_position += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => panic!("Invalid instruction {}", instruction)
        }
    }
    h_position * depth
}


fn follow_course_part_b(instructions: &[(String, i32)]) -> i32 {
    let mut h_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (instruction, value) in instructions {
        match &instruction[..] {
            "forward" => {
                h_position += value;
                depth += aim * value
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("Invalid instruction {}", instruction)
        }
    }
    h_position * depth
}
