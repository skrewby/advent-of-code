mod input;
mod aoc2015;
mod solution;

use anyhow::Result;

fn main() -> Result<()> {
    let input = input::get_input(2015, 1)?;
    let solution = aoc2015::solve(1, input)?;

    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);

    Ok(())
}
