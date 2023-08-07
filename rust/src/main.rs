mod aoc2015;
mod input;
mod solution;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let year = 2015;
    let day = 7;
    let input = input::get_input(year, day).context("Getting input for solution")?;

    let solution = match year {
        2015 => aoc2015::solve(day, input).context("Solving AoC 2015 problem")?,
        _ => panic!("Year {} not implemented yet!", year),
    };

    println!("Time taken: {} us", solution.duration.as_micros());
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);

    Ok(())
}
