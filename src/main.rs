mod day_01;

use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => solve_day(args[1].trim().parse().expect("You must enter a number between 1 and 25.")),
        _ => invalid_input()
    }
}


fn solve_day(day: i32) {
    if !(1..=25).contains(&day) {
        invalid_input();
    }
    let fn_day = match day {
        1 => day_01::day_01,
        _ => unimplemented!(),
    };
    println!("# Processing Day {} :", day);
    fn_day()
}

fn invalid_input() {
    panic!("You must enter a number between 1 and 25.")
}
