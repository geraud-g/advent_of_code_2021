use advent_of_code_2021::utils::inputs::get_file;


pub fn day_01() {
    let measurements = get_input();

    let solution_a = count_measurements_increases(&measurements, 1);
    println!("Solution part A is: {}", solution_a);

    let solution_b = count_measurements_increases(&measurements, 3);
    println!("Solution part B is: {}", solution_b);
}


fn get_input() -> Vec<i32> {
    let data = get_file("./src/day_01/input.txt");
    data.lines().map(|s| s.parse().unwrap()).collect()
}


fn count_measurements_increases(measurements: &[i32], range: usize) -> usize {
    (0..(measurements.len() - range))
        .filter(|idx| is_lower_than_next_window(measurements, range, *idx))
        .count()
}


fn is_lower_than_next_window(measurements: &[i32], range: usize, idx: usize) -> bool {
    let current_measurement = sum_range(measurements, range, idx);
    let next_measurement = sum_range(measurements, range, idx + 1);
    current_measurement < next_measurement
}


fn sum_range(measurements: &[i32], range: usize, idx: usize,) -> i32 {
    measurements[idx..(idx + range)].iter().sum()
}
