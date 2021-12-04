use advent_of_code_2021::utils::helpers::get_char;
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

    for col_idx in 0..line_len {
        let nbr_ones = count_ones_in_col(diagnostic_report, col_idx);
        if nbr_ones > diagnostic_report.len() / 2 {
            gamma |= 1 << (line_len - col_idx - 1);
        }
    }
    gamma
}


/// Swap `gamma_rate` bits, and truncate to keep only the first `line_len` bits
fn get_epsilon_rate(gamma_rate: i32, line_len: usize) -> i32 {
    !gamma_rate & !(!0 << line_len)
}


fn get_life_support_rating(diagnostic_report: &[String], line_len: usize) -> i32 {
    let oxygen_generator_filter= |nbr_ones, nbr_zeroes| if nbr_ones >= nbr_zeroes { '1' } else { '0' };
    let co2_scrubber_filter = |nbr_ones, nbr_zeroes| if nbr_ones >= nbr_zeroes { '0' } else { '1' };

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
            .filter(|l| get_char(l, col_idx).eq(&filter_start_by))
            .map(|s| s.to_string()).collect();
    }
    i32::from_str_radix(&lines_to_eval[0], 2).unwrap()
}


fn count_ones_in_col(diagnostic_report: &[String], col_idx: usize) -> usize {
    diagnostic_report.iter()
        .map(|line| get_char(&line, col_idx))
        .filter(|char| *char == '1')
        .count()
}
