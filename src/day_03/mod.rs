use advent_of_code_2021::utils::inputs::get_file;

pub fn day_03() {
    let diagnostic_report = get_input();
    let line_len = diagnostic_report[0].len();

    let solution_a = get_power_consumption(&diagnostic_report, line_len);
    println!("Solution for Day 3, part A is: {}", solution_a);


    let solution_b = get_life_support_rating(&diagnostic_report, line_len);
    println!("Solution for Day 3, part B is: {}", solution_b);
}


fn get_input() -> Vec<String> {
    get_file("./src/day_03/input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}


fn get_power_consumption(diagnostic_report: &[String], line_len: usize) -> i32 {
    let gamma_rate = get_gamma_rate(diagnostic_report, line_len);
    gamma_rate * get_epsilon_rate(gamma_rate, line_len)
}


fn get_gamma_rate(diagnostic_report: &[String], line_len: usize) -> i32 {
    let mut gamma: i32 = 0;

    for x in 0..line_len {
        let one_count = count_ones_in_col(diagnostic_report, x);
        if one_count > diagnostic_report.len() / 2 {
            gamma |= 1 << (line_len - x - 1);
        }
    }
    gamma
}


/// Swap `gamma_rate` bits, and truncate to keep only the first `line_len` bits
fn get_epsilon_rate(gamma_rate: i32, line_len: usize) -> i32 {
    !gamma_rate & !(!0 << line_len)
}


fn get_life_support_rating(diagnostic_report: &[String], line_len: usize) -> i32 {
    let oxygen_generator_rating = get_rating(diagnostic_report, line_len, &oxygen_generator_filter);
    let co2_scrubber_rating = get_rating(diagnostic_report, line_len, &co2_scrubber_filter);
    oxygen_generator_rating * co2_scrubber_rating
}


fn get_rating(diagnostic_report: &[String], line_len: usize, filter: &dyn Fn(usize, usize) -> char) -> i32 {
    let mut lines_to_eval = diagnostic_report.to_vec();

    for col_idx in 0..line_len {
        if lines_to_eval.len() == 1 {
            break;
        }
        let one_count = count_ones_in_col(&lines_to_eval, col_idx);
        let filter_start_by = filter(one_count, lines_to_eval.len() - one_count);
        lines_to_eval = lines_to_eval.iter()
            .filter(|l| l.chars().nth(col_idx).unwrap().eq(&filter_start_by))
            .map(|s| s.to_string()).collect();
    }
    i32::from_str_radix(&lines_to_eval[0], 2).unwrap()
}


fn oxygen_generator_filter(one_count: usize, zero_count: usize) -> char {
    if one_count >= zero_count { '1' } else { '0' }
}


fn co2_scrubber_filter(one_count: usize, zero_count: usize) -> char {
    if one_count >= zero_count { '0' } else { '1' }
}


fn count_ones_in_col(diagnostic_report: &[String], col_idx: usize) -> usize {
    (0..diagnostic_report.len())
        .map(|y| diagnostic_report[y].chars().nth(col_idx).unwrap())
        .filter(|char| *char == '1')
        .count()
}
