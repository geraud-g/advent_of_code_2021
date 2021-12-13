use std::collections::{HashMap, HashSet};

use advent_of_code_2021::utils::inputs::get_file;

pub fn day_12() {
    let caves = get_input();

    let solution_a = count_paths(&caves);
    println!("Solution for Day 12, part A is: {}", solution_a);


    let solution_b = count_paths_part_2(&caves);
    println!("Solution for Day 12, part B is: {}", solution_b);
}


fn get_input() -> HashMap<String, Vec<String>> {
    let mut caves = HashMap::new();
    for line in get_file("./src/day_12/input.txt").lines() {
        let split: Vec<_> = line.trim().split('-').map(|l| l.to_string()).collect();
        caves.entry(split[0].clone()).or_insert_with(Vec::new).push(split[1].clone());
        caves.entry(split[1].clone()).or_insert_with(Vec::new).push(split[0].clone());
    }
    caves
}


fn count_paths(caves: &HashMap<String, Vec<String>>) -> usize {
    let start = String::from("start");
    let mut paths_list = vec![];
    let current_path = vec![&start];
    rec_explore(caves, &start, &current_path, &mut paths_list);
    paths_list.len()
}


fn rec_explore<'a>(
    caves: &'a HashMap<String, Vec<String>>,
    current_cave: &str,
    current_path: &[&'a String],
    path_list: &mut Vec<Vec<&'a String>>) {
    let end = String::from("end");
    if current_cave.eq(&end) {
        path_list.push(current_path.to_vec());
        return;
    }

    for neighbour in caves.get(current_cave).unwrap() {
        if is_upper(neighbour) || !current_path.contains(&neighbour) {
            let mut new_path = current_path.to_vec();
            new_path.push(neighbour);
            rec_explore(caves, neighbour, &new_path, path_list)
        }
    }
}


fn count_paths_part_2(caves: &HashMap<String, Vec<String>>) -> usize {
    let start = String::from("start");
    let current_path = vec![&start];
    let mut all_path = HashSet::new();

    for key in caves.keys() {
        if key.eq("start") || key.eq("end") || is_upper(key) {
            continue;
        }
        let mut paths_list = vec![];
        rec_explore_part_2(caves, &start, &current_path, &mut paths_list, key);
        all_path.extend(paths_list)
    }
    all_path.len()
}


fn rec_explore_part_2<'a>(
    caves: &'a HashMap<String, Vec<String>>,
    current_cave: &str,
    current_path: &[&'a String],
    path_list: &mut Vec<Vec<&'a String>>,
    double_cave: &'a str,
) {
    let end = String::from("end");
    if current_cave.eq(&end) {
        path_list.push(current_path.to_vec());
        return;
    }

    for neighbour in caves.get(current_cave).unwrap() {
        if is_upper(neighbour)
            || !current_path.contains(&neighbour)
            || (neighbour.eq(double_cave) && current_path.iter().filter(|&p| p.eq(&neighbour)).count() < 2) {
            let mut new_path = current_path.to_vec();
            new_path.push(neighbour);
            rec_explore_part_2(caves, neighbour, &new_path, path_list, double_cave)
        }
    }
}


fn is_upper(string: &str) -> bool {
    string.chars().next().unwrap().is_uppercase()
}