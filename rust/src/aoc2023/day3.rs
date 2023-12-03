use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use crate::solution::Solution;
use anyhow::{Context, Result};
use regex::Regex;

fn is_valid_index(ch_x: usize, ch_y: usize, schema: &Vec<Vec<char>>) -> bool {
    if !schema[ch_y][ch_x].is_digit(10) {
        return false;
    }

    for dy in -1..2 {
        if (dy + ch_y as isize) < 0 || (dy + ch_y as isize) >= schema.len().try_into().unwrap() {
            continue;
        }
        let y = ch_y as isize + dy;
        let y = y as usize;

        for dx in -1..2 {
            if (dx + ch_x as isize) < 0
                || (dx + ch_x as isize) >= schema[0].len().try_into().unwrap()
            {
                continue;
            }
            let x = ch_x as isize + dx;
            let x = x as usize;

            let schema_char = schema[y][x];
            if schema_char != '.' && !schema_char.is_digit(10) {
                return true;
            }
        }
    }

    false
}

fn get_gear_ratio(
    ch_x: usize,
    ch_y: usize,
    schema: &Vec<Vec<char>>,
    parts: &HashMap<(usize, usize), u32>,
) -> u32 {
    let mut ratio = 0;
    let mut part_set: HashSet<(usize, usize)> = HashSet::new();
    for dy in -1..2 {
        if (dy + ch_y as isize) < 0 || (dy + ch_y as isize) >= schema.len().try_into().unwrap() {
            continue;
        }
        let y = ch_y as isize + dy;
        let y = y as usize;

        for dx in -1..2 {
            if (dx + ch_x as isize) < 0
                || (dx + ch_x as isize) >= schema[0].len().try_into().unwrap()
            {
                continue;
            }
            let x = ch_x as isize + dx;
            let x = x as usize;

            for ((part_x, part_y), num) in parts {
                if *part_y != y {
                    continue;
                }
                let count = successors(Some(*num), |&n| (n >= 10).then(|| n / 10)).count();
                if x >= *part_x && x < *part_x + count {
                    part_set.insert((*part_x, *part_y));
                }
            }
        }
    }

    if part_set.len() >= 2 {
        let mut current_ratio = 1;
        for (part_x, part_y) in part_set {
            current_ratio *= parts.get(&(part_x, part_y)).unwrap();
        }
        ratio += current_ratio;
    }

    ratio
}

fn get_part_numbers(input: &str, schema: &Vec<Vec<char>>) -> Result<HashMap<(usize, usize), u32>> {
    let get_numbers_regex = Regex::new(r"([0-9]+)").unwrap();
    let mut parts: HashMap<(usize, usize), u32> = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        for (index, [number]) in get_numbers_regex
            .captures_iter(row)
            .map(|c| (c.get(0).unwrap().start(), c.extract().1))
        {
            let size = (number as &str).len();
            for x in index..size + index {
                if is_valid_index(x, y, schema) {
                    let coord = (index, y);
                    parts.insert(coord, u32::from_str_radix(number as &str, 10)?);
                    break;
                }
            }
        }
    }

    Ok(parts)
}

fn get_total_gear_ration(
    input: &str,
    schema: &Vec<Vec<char>>,
    parts: &HashMap<(usize, usize), u32>,
) -> Result<u32> {
    let get_total_gear_ratio_regex = Regex::new(r"([*]+)").unwrap();
    let mut total_gear_ratio = 0;

    for (y, row) in input.lines().enumerate() {
        for x in get_total_gear_ratio_regex
            .captures_iter(row)
            .map(|c| c.get(0).unwrap().start())
        {
            total_gear_ratio += get_gear_ratio(x, y, schema, parts);
        }
    }

    Ok(total_gear_ratio)
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let schema: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Represents the schema but will have boolean in each coord to represent if that spae is taken
    // by a character
    let part_numbers = get_part_numbers(&input, &schema).context("Searching for part numbers")?;
    let sum_part_nums: u32 = part_numbers
        .iter()
        .map(|(_, part_number)| part_number)
        .sum();
    solution.part1 = sum_part_nums.to_string();
    solution.part2 = get_total_gear_ration(&input, &schema, &part_numbers)
        .context("Get total gear ratio")?
        .to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_part_numbers_test() -> Result<()> {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let schema: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let part_numbers = get_part_numbers(&input, &schema)?;
        let sum_part_nums: u32 = part_numbers
            .iter()
            .map(|(_, part_number)| part_number)
            .sum();
        assert_eq!(sum_part_nums, 4361);
        Ok(())
    }

    #[test]
    fn sum_gear_ratios_test() -> Result<()> {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let schema: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let parts = get_part_numbers(&input, &schema)?;
        let ratio = get_total_gear_ration(&input, &schema, &parts)?;

        assert_eq!(ratio, 467835);
        Ok(())
    }
}
