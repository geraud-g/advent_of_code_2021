use advent_of_code_2021::utils::inputs::get_file;

pub fn day_09() {
    let lava_tubes = get_input();

    let solution_a = sum_risk_level(&lava_tubes);
    assert_eq!(solution_a, 512);
    println!("Solution for Day 9, part A is: {}", solution_a);
}


fn get_input() -> Vec<Vec<u32>> {
    get_file("./src/day_09/input.txt").lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}


fn sum_risk_level(tubes: &[Vec<u32>]) -> u32 {
    let height = tubes.len();
    let width = tubes[0].len();
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if is_lowest_point(tubes, height, width, y, x) {
                count += tubes[y][x] + 1;
            }
        }
    }
    count
}


fn is_lowest_point(tubes: &[Vec<u32>], height: usize, width: usize, y: usize, x: usize) -> bool {
    let val = tubes[y][x];

    if y > 0 && tubes[y - 1][x] <= val {
        return false;
    }
    if y < (height - 1) && tubes[y + 1][x] <= val {
        return false;
    }
    if x > 0 && tubes[y][x - 1] <= val {
        return false;
    }
    if x < (width - 1) && tubes[y][x + 1] <= val {
        return false;
    }
    true
}


