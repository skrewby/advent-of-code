use crate::solution::Solution;
use anyhow::Result;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

pub fn solve(day: u32, input: String) -> Result<Solution> {
    let mut solution = Solution::new();

    match day {
        1 => day1::solve(input, &mut solution)?,
        2 => day2::solve(input, &mut solution)?,
        3 => day3::solve(input, &mut solution)?,
        4 => day4::solve(input, &mut solution)?,
        5 => day5::solve(input, &mut solution)?,
        6 => day6::solve(input, &mut solution)?,
        7 => day7::solve(input, &mut solution)?,
        8 => day8::solve(input, &mut solution)?,
        9 => day9::solve(input, &mut solution)?,
        10 => day10::solve(input, &mut solution)?,
        _ => println!("Solution for day {} doesn't exist", day),
    }

    solution.complete();
    Ok(solution)
}
