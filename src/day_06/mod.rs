use advent_of_code_2021::utils::inputs::get_file;

pub fn day_06() {
    let fish_list = get_input();

    let solution_a = get_nbr_lanternfish(&fish_list, 80);
    println!("Solution for Day 6, part A is: {}", solution_a);

    let solution_b = get_nbr_lanternfish(&fish_list, 256);
    println!("Solution for Day 6, part B is: {}", solution_b);
}


fn get_input() -> [u64; 9] {
    let int_list = get_file("./src/day_06/input.txt")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut fish_list = [0; 9];
    for val in int_list {
        fish_list[val] += 1;
    }
    fish_list
}


fn get_nbr_lanternfish(initial_fish_list: &[u64; 9], iterations: usize) -> u64 {
    let mut fish_list = *initial_fish_list;

    for _ in 0..iterations {
        let nbr_zero_fish = fish_list[0];
        for i in 0..8 {
            fish_list[i] = fish_list[i + 1]
        }
        fish_list[8] = nbr_zero_fish;
        fish_list[6] += nbr_zero_fish;
    }
    fish_list.iter().sum()
}
