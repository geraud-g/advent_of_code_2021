use advent_of_code_2021::utils::inputs::get_file;

pub fn day_07() {
    let input = get_input();

    let solution_a = solve_part_a(&input);
    println!("Solution for Day 6, part A is: {}", solution_a);

    let solution_b = solve_part_b(&input);
    println!("Solution for Day 6, part B is: {}", solution_b);
}


fn get_input() -> Vec<i32> {
    get_file("./src/day_07/input.txt")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}


fn solve_part_a(crabs: &[i32]) -> i32 {
    (0..*(crabs.iter().max().unwrap()))
        .map(|pos| get_movement_cost(crabs, &pos))
        .min()
        .unwrap()
}

fn get_movement_cost(crabs: &[i32], position: &i32) -> i32 {
    crabs.iter()
        .map(|crab| (crab - position).abs())
        .sum()
}


fn solve_part_b(crabs: &[i32]) -> i32 {
    let crab_sum = crabs.iter().sum::<i32>();
    let crab_len = crabs.len() as i32;
    let mean = crab_sum / crab_len;
    let kek = vec![get_movement_cost_b(crabs, &mean), get_movement_cost_b(crabs, &(mean - 1)), get_movement_cost_b(crabs, &(mean + 1))];
    *kek.iter().min().unwrap()
}

fn get_movement_cost_b(crabs: &[i32], position: &i32) -> i32 {
    crabs.iter()
        .map(|crab| (1..((crab - position).abs() + 1)).sum::<i32>())
        .sum()
}