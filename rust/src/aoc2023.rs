use crate::solution::Solution;
use anyhow::Result;

mod utils;

mod day1;
mod day2;

pub fn solve(day: u32, input: String) -> Result<Solution> {
    let mut solution = Solution::new();

    match day {
        1 => day1::solve(input, &mut solution)?,
        2 => day2::solve(input, &mut solution)?,
        _ => println!("Solution for day {} doesn't exist", day),
    }

    Ok(solution)
}
