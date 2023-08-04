use anyhow::Result;
use crate::solution::Solution;

mod day1;

pub fn solve(day: u32, input: String) -> Result<Solution> {
    let mut solution = Solution::new();

    match day {
        1 => day1::solve(&mut solution)?,
        _ => println!("Solution for day {} doesn't exist", day)
    }

    solution.complete();
    Ok(solution)
}

