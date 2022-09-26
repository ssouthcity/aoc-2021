use crate::problem::Problem;

pub struct DayOne {}

impl Problem for DayOne {
    fn a(&self, input: &String) -> String {
        let readings: Vec<u16> = input.lines().map(|l| l.parse::<u16>().unwrap()).collect();
        let mut increments: u32 = 0;

        for i in 1..readings.len() {
            if readings[i] > readings[i - 1] {
                increments += 1;
            }
        }

        format!("sonar sweep recorded {} increments", increments)
    }

    fn b(&self, input: &String) -> String {
        let readings: Vec<u16> = input.lines().map(|l| l.parse::<u16>().unwrap()).collect();
        let mut increments: u32 = 0;

        for i in 3..readings.len() {
            if readings[i] > readings[i - 3] {
                increments += 1;
            }
        }

        format!(
            "sonar sweep recorded {} increments with sliding windows",
            increments
        )
    }
}
