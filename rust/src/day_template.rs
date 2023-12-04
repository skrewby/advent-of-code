use crate::solution::Solution;
use anyhow::{Context, Result};

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = 0;
    solution.part2 = 0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() -> Result<()> {
        Ok(())
    }
}
