use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;

use advent_of_code_2021::utils::inputs::get_file;


pub fn day_08() {
    let numbers = get_input();

    let solution_a = solve_part_a(&numbers);
    assert_eq!(solution_a, 237);
    println!("Solution for Day 8, part A is: {}", solution_a);

    let solution_b = solve_part_b(&numbers);
    assert_eq!(solution_b, 1009098);
    println!("Solution for Day 8, part B is: {}", solution_b);
}


fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    get_file("./src/day_08/input.txt")
        .lines()
        .map(|line| get_numbers(line))
        .collect()
}


fn get_numbers(line: &str) -> (Vec<String>, Vec<String>) {
    let convert = |l: &str| l.trim().split(' ').map(|s| s.to_string()).collect();
    let mut split_line = line.split('|');
    (convert(split_line.next().unwrap()), convert(split_line.next().unwrap()))
}


fn solve_part_a(numbers: &[(Vec<String>, Vec<String>)]) -> usize {
    numbers.iter()
        .flat_map(|(_, b)| b)
        .filter(|n| (n.len() >= 2 && n.len() <= 4) || n.len() == 7).count()
}


fn solve_part_b(numbers: &[(Vec<String>, Vec<String>)]) -> usize {
    numbers.iter().map(|n| solve_line(n)).sum()
}


fn solve_line(line: &(Vec<String>, Vec<String>)) -> usize {
    let patterns = &line.0;
    let mut found_patterns: HashMap<usize, &str> = HashMap::new();


    // 1, 4, 7, 8 have unique len
    found_patterns.insert(1, patterns.iter().find(|p| p.len() == 2).unwrap());
    found_patterns.insert(4, patterns.iter().find(|p| p.len() == 4).unwrap());
    found_patterns.insert(7, patterns.iter().find(|p| p.len() == 3).unwrap());
    found_patterns.insert(8, patterns.iter().find(|p| p.len() == 7).unwrap());

    // 6 is the only number of len 6 which does not contains 1
    let six = patterns.iter().find(|p| p.len() == 6 && !contains_all(p, get( 1, &found_patterns))).unwrap();
    found_patterns.insert(6, six);

    // 9 is len 6 and contains 4
    let nine = patterns.iter().find(|p| p.len() == 6 && contains_all(p, get( 4, &found_patterns))).unwrap();
    found_patterns.insert(9, nine);

    // 0 is the remaining number of len 6
    let zero = patterns.iter()
        .find(|p| p.len() == 6 && !p.eq(&get( 6, &found_patterns)) && !p.eq(&get( 9, &found_patterns))
        ).unwrap();
    found_patterns.insert(0, zero);

    //3 is len 5 and contains 7
    let three = patterns.iter().find(|p| p.len() == 5 && contains_all(p, get( 7, &found_patterns))).unwrap();
    found_patterns.insert(3, three);

    //5 is len 5 and is contained by 6
    let five = patterns.iter().find(|p| p.len() == 5 && contains_all(get( 6, &found_patterns), p)).unwrap();
    found_patterns.insert(5, five);


    // 2 is the remaining number of len 5
    let two = patterns.iter()
        .find(|p|
            p.len() == 5
                && !p.eq(&get( 3, &found_patterns))
                && !p.eq(&get( 5, &found_patterns))
        ).unwrap();
    found_patterns.insert(2, two);

    decode_lines(&line.1, &found_patterns)
}


fn contains_all(val: &str, sub_string: &str) -> bool {
    sub_string.chars().all(|c| val.contains(c))
}

fn get<'a>(val: usize, hashmap: &'a HashMap<usize, &str>) -> &'a str {
    hashmap.get(&val).unwrap()
}

fn decode_lines(lines: &[String], found_patterns: &HashMap<usize, &str>) -> usize {
    let mut string = String::new();
    for num in lines {
        for (key, val) in found_patterns {
            if val.chars().sorted().eq(num.chars().sorted()) {
                string = string.add(&key.to_string());
            }
        }
    }
    usize::from_str(&string).unwrap()
}