use std::collections::HashSet;

use regex::Regex;

use advent_of_code_2021::utils::inputs::{get_file, LINE_ENDING};

const WIDTH: usize = 5;
const BOARD_SIZE: usize = 25;


#[derive(Debug, Default, Eq, PartialEq)]
struct Board {
    pub id: usize,
    pub numbers: Vec<u8>,
    pub marked: HashSet<u8>,
}


impl Board {
    pub fn sum_unmarked(&self) -> u32 {
        self.numbers.iter().filter(|&n| !self.marked.contains(n)).map(|n| *n as u32).sum()
    }

    pub fn won(&self) -> bool {
        self.won_horizontal() || self.won_vertical()
    }

    fn won_vertical(&self) -> bool {
        (0..WIDTH)
            .any(|idx| (idx..BOARD_SIZE).step_by(WIDTH).all(|i| self.marked.contains(&self.numbers[i])))
    }

    fn won_horizontal(&self) -> bool {
        (0..BOARD_SIZE).step_by(WIDTH)
            .any(|idx| (idx..(idx + WIDTH)).all(|i| self.marked.contains(&self.numbers[i])))
    }
}


pub fn day_04() {
    let (numbers, mut boards) = get_input();
    let solution_a = play_bingo(&numbers, &mut boards, 1);
    assert_eq!(solution_a, 25410);
    println!("Solution for Day 4, part A is: {}", solution_a);

    let (numbers, mut boards) = get_input();
    let nbr_boards = boards.len();
    let solution_b = play_bingo(&numbers, &mut boards, nbr_boards);
    assert_eq!(solution_b, 2730);
    println!("Solution for Day 4, part B is: {}", solution_b);
}


fn get_input() -> (Vec<u8>, Vec<Board>) {
    let re = Regex::new(r"\s+").unwrap();
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let file = get_file("./src/day_04/input.txt");
    let mut file = file.split(&split_separator).map(|s| s.to_string());
    let numbers = file.next().unwrap().split(',').map(|str| str.parse::<u8>().unwrap()).collect();
    let mut boards = vec![];

    for (idx, line) in file.enumerate() {
        let mut new_board = Board{id: idx, ..Default::default()};
        let line = re.replace_all(&line.trim(), " ");
        for val in line.split(' ') {
            new_board.numbers.push(val.parse::<u8>().unwrap());
        }
        boards.push(new_board);
    }
    (numbers, boards)
}


fn play_bingo(numbers: &[u8], boards: &mut [Board], nth_victory: usize) -> u32 {
    let mut wins = HashSet::new();

    for number in numbers {
        for board in boards.iter_mut() {
            play_board(board, &mut wins, number);
            if wins.len() == nth_victory {
                return *number as u32 * board.sum_unmarked();
            }
        }
    }
    panic!("Solution not found")
}


fn play_board(board: &mut Board, wins: &mut HashSet<usize>, number: &u8) {
    if !board.numbers.contains(&number) || wins.contains(&board.id) {
        return;
    }
    board.marked.insert(*number);
    if board.won() {
        wins.insert(board.id);
    }
}
