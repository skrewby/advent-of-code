use crate::solution::Solution;
use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn solve(day: u32, input: String) -> Result<Solution> {
    let mut solution = Solution::new();

    match day {
        1 => day1::solve(input, &mut solution)?,
        2 => day2::solve(input, &mut solution)?,
        3 => day3::solve(input, &mut solution)?,
        4 => day4::solve(input, &mut solution)?,
        5 => day5::solve(input, &mut solution)?,
        6 => day6::solve(input, &mut solution)?,
        _ => println!("Solution for day {} doesn't exist", day),
    }

    solution.complete();
    Ok(solution)
}
