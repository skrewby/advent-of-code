use crate::solution::Solution;
use anyhow::{Context, Result};

fn travel_direction(cmd: char) -> i32 {
    match cmd {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn find_basement_entry(input: &str) -> Result<i32> {
    let mut floor = 0;

    for (i, cmd) in input.chars().enumerate() {
        floor += travel_direction(cmd);

        if floor < 0 {
            return Ok(i as i32 + 1);
        }
    }

    Ok(0)
}

fn travel_floors(input: &str) -> Result<i32> {
    let mut floor = 0;

    for cmd in input.chars() {
        floor += travel_direction(cmd);
    }

    Ok(floor)
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = travel_floors(&input)
        .context("Solving Year 2015 Day 1")?
        .to_string();
    solution.part2 = find_basement_entry(&input)
        .context("Solving Year 2015 Day 2")?
        .to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn travel() -> Result<()> {
        assert_eq!(travel_floors("(())")?, 0);
        assert_eq!(travel_floors("(((")?, 3);
        assert_eq!(travel_floors("))(((((")?, 3);
        assert_eq!(travel_floors("())")?, -1);
        assert_eq!(travel_floors(")())())")?, -3);

        Ok(())
    }

    #[test]
    fn find_basement_index() -> Result<()> {
        assert_eq!(find_basement_entry(")")?, 1);
        assert_eq!(find_basement_entry("()())")?, 5);

        Ok(())
    }
}
