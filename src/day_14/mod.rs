use std::collections::HashMap;

use itertools::Itertools;

use advent_of_code_2021::utils::inputs::{get_file, LINE_ENDING};

pub fn day_14() {
    let (polymer_template, insertion_rules) = get_input();

    let solution_a = solve(&polymer_template, &insertion_rules, 10);
    println!("Solution for Day 14, part A is: {}", solution_a);

    let solution_b = solve(&polymer_template, &insertion_rules, 40);
    println!("Solution for Day 14, part B is: {}", solution_b);
}


type Pair = (char, char);


fn get_input() -> (Vec<char>, HashMap<Pair, char>) {
    let split_val = &format!("{}{}", LINE_ENDING, LINE_ENDING);
    let data = get_file("./src/day_14/input.txt");
    let mut data = data.split(split_val);
    let polymer_template = data.next().unwrap().chars().collect();
    let mut rules = HashMap::new();
    for line in data.next().unwrap().lines() {
        let mut split_line = line.trim().split(" -> ");
        let mut key = split_line.next().unwrap().chars();
        let mut val = split_line.next().unwrap().chars();
        rules.insert((key.next().unwrap(), key.next().unwrap()), val.next().unwrap());
    }
    (polymer_template, rules)
}


fn solve(polymer_template: &[char], insertion_rules:&HashMap<Pair, char>, iterations: usize) -> usize {
    let mut pairs_count = polymer_to_hash_count(polymer_template);
    let (fist_char, last_char) = (polymer_template.first().unwrap(), polymer_template.last().unwrap());

    for _ in 0..iterations {
        pairs_count = apply_rules(&pairs_count, insertion_rules)
    }
    count_elements(&pairs_count, *fist_char,*last_char)
}


fn polymer_to_hash_count(polymer_template: &[char]) -> HashMap<Pair, usize> {
    let pairs: Vec<Pair> = polymer_template.iter()
        .tuple_windows::<(_, _)>()
        .map(|c|(*c.0, *c.1))
        .collect();
    let mut count_map = HashMap::new();
    for pair in pairs {
        *count_map.entry(pair).or_insert(0) += 1;
    }
    count_map
}


fn apply_rules(polymer: &HashMap<Pair, usize>, insertion_rules: &HashMap<Pair, char>) -> HashMap<Pair, usize> {
    let mut new_polymer = HashMap::new();
    for (pair, count) in polymer {
        if let Some(new_elem) = insertion_rules.get(pair) {
            *new_polymer.entry((pair.0, *new_elem)).or_insert(0) += count;
            *new_polymer.entry((*new_elem, pair.1)).or_insert(0) += count;
        }
    }
    new_polymer
}


fn count_elements(polymer: &HashMap<Pair, usize>, first_char: char, last_char: char) -> usize{
    let mut element_count = HashMap::new();
    for (key, count) in polymer {
        *element_count.entry(key.0).or_insert(0) += count;
        *element_count.entry(key.1).or_insert(0) += count;
    }
    *element_count.entry(first_char).or_insert(0) += 1;
    *element_count.entry(last_char).or_insert(0) += 1;

    let list_count: Vec<usize> = element_count.values().map(|v|v/2).sorted().collect();
    *list_count.last().unwrap() - *list_count.first().unwrap()
}
