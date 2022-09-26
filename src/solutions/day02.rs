use std::str::FromStr;

use crate::problem::Problem;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, val) = s
            .split_once(' ')
            .expect("string must be composed of a direction and a value, split by a space");

        let numval: i32 = val
            .parse()
            .expect("value following the string has to be a valid i32");

        match (dir, numval) {
            ("forward", d) => Ok(Self::Forward(d)),
            ("up", d) => Ok(Self::Up(d)),
            ("down", d) => Ok(Self::Down(d)),
            _ => Err("invalid command"),
        }
    }
}

pub struct DayTwo {}

impl Problem for DayTwo {
    fn a(&self, input: &String) -> String {
        let commands: Vec<Command> = input.lines().map(|l| l.parse().unwrap()).collect();

        let mut position: i32 = 0;
        let mut depth: i32 = 0;

        for c in commands {
            match c {
                Command::Forward(d) => position += d,
                Command::Down(d) => depth += d,
                Command::Up(d) => depth -= d,
            };
        }

        format!("submarine dove by {} units", position * depth)
    }

    fn b(&self, input: &String) -> String {
        let commands: Vec<Command> = input.lines().map(|l| l.parse().unwrap()).collect();

        let mut position: i32 = 0;
        let mut depth: i32 = 0;
        let mut aim: i32 = 0;

        for c in commands {
            match c {
                Command::Up(d) => aim -= d,
                Command::Down(d) => aim += d,
                Command::Forward(d) => {
                    position += d;
                    depth += aim * d;
                }
            }
        }

        format!("submarine dove by {} units using aim", position * depth)
    }
}
