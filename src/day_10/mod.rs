use advent_of_code_2021::utils::inputs::get_file;

pub fn day_10() {
    let navigation_subsystem = get_input();

    let solution_a = get_total_corruption_score(&navigation_subsystem);
    println!("Solution for Day 10, part A is: {}", solution_a);


    let solution_b = get_total_completion_score(&navigation_subsystem);
    println!("Solution for Day 10, part B is: {}", solution_b);
}


fn get_input() -> Vec<String> {
    get_file("./src/day_10/input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}


fn get_total_corruption_score(navigation_subsystem: &[String]) -> i32 {
    navigation_subsystem.iter()
        .map(|l| get_line_corruption_score(l))
        .sum()
}


fn get_line_corruption_score(line: &str) -> i32 {
    let mut stack = vec![];

    for chunk in line.trim().chars() {
        if "([{<".contains(chunk) {
            stack.push(chunk)
        } else if is_valid_closing_chunk(stack.last().unwrap(), &chunk) {
            stack.pop();
        } else {
            return match chunk {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Wrong closing chunk: {}", chunk)
            };
        }
    }
    0
}

fn is_valid_closing_chunk(opening_chunk: &char, &closing_chunk: &char) -> bool {
    match opening_chunk {
        '(' => closing_chunk.eq(&')'),
        '[' => closing_chunk.eq(&']'),
        '{' => closing_chunk.eq(&'}'),
        '<' => closing_chunk.eq(&'>'),
        _ => false
    }
}


fn get_total_completion_score(navigation_subsystem: &[String]) -> u64 {
    let mut completion_scores: Vec<_> = navigation_subsystem.iter()
        .map(|line| get_line_completion_score(line))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect();
    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}


fn get_line_completion_score(line: &str) -> Option<u64> {
    let mut stack = vec![];

    for chunk in line.trim().chars() {
        if "([{<".contains(chunk) {
            stack.push(chunk)
        } else if is_valid_closing_chunk(stack.last().unwrap(), &chunk) {
            stack.pop();
        } else {
            return None;
        }
    }

    let mut completion_score = 0;
    while let Some(chunk) = stack.pop() {
        completion_score = completion_score * 5 + match chunk {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Wrong opening chunk: {}", chunk)
        };
    }

    Some(completion_score)
}
