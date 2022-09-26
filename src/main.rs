use lazy_static::lazy_static;

use std::{env, fs};

lazy_static! {
    static ref SOLUTIONS: [Box<dyn aoc_2021::problem::Problem + Sync>; 1] =
        [Box::new(aoc_2021::solutions::day01::DayOne {})];
}

fn main() {
    let day: usize = env::args()
        .nth(1)
        .expect("too few arguments")
        .parse()
        .expect("argument must be an integer");

    let input_fpath = format!("src/inputs/day{:0>2}.txt", day);

    println!("{}", input_fpath);

    let input = fs::read_to_string(input_fpath).expect("unable to read input file");

    let problem = SOLUTIONS.get(day - 1).expect("invalid day");

    println!("a {}", problem.a(&input));
    println!("b {}", problem.b(&input));
}
