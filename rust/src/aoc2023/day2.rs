use crate::solution::Solution;
use anyhow::{Context, Result};
use std::cmp;
use std::collections::LinkedList;

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: LinkedList<Set>,
}

fn parse_game(line: &str) -> Result<Game> {
    let game_line: Vec<&str> = line.split(':').collect();

    let (_, game_id) = game_line[0].split_once(' ').unwrap();

    let mut sets: LinkedList<Set> = LinkedList::new();
    for set in game_line[1].split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cubes in set.split(',') {
            let cubes = cubes.trim();
            let cube_line: Vec<&str> = cubes.split(' ').collect();
            let num = u32::from_str_radix(cube_line[0], 10)?;
            match cube_line[1] {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => {}
            }
        }
        sets.push_back(Set { red, green, blue });
    }

    Ok(Game {
        id: u32::from_str_radix(game_id, 10)?,
        sets,
    })
}

fn part_2(input: &str) -> Result<String> {
    let mut total_power = 0;

    for line in input.lines() {
        let game = parse_game(line)?;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in game.sets {
            max_red = cmp::max(max_red, set.red);
            max_green = cmp::max(max_green, set.green);
            max_blue = cmp::max(max_blue, set.blue);
        }
        total_power += max_red * max_green * max_blue;
    }

    Ok(total_power.to_string())
}

fn part_1(input: &str) -> Result<String> {
    let mut valid_games = 0;

    for line in input.lines() {
        let game = parse_game(line)?;
        let mut valid = true;
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                valid = false;
                break;
            }
        }

        if valid {
            valid_games += game.id;
        }
    }

    Ok(valid_games.to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input).context("Solving Year 2023 Day 2 Part 1")?;
    solution.part2 = part_2(&input).context("Solving Year 2023 Day 2 Part 2")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_test() -> Result<()> {
        let game1 = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?;
        assert_eq!(game1.id, 1);
        assert_eq!(game1.sets.front().unwrap().red, 4);
        assert_eq!(game1.sets.front().unwrap().blue, 3);
        assert_eq!(game1.sets.front().unwrap().green, 0);
        Ok(())
    }
}
