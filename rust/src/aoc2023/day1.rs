use crate::aoc2023::utils::trie::Trie;
use crate::solution::Solution;
use anyhow::{Context, Result};

fn get_digits(line: &str) -> (u32, u32) {
    let mut digits = Vec::new();

    for ch in line.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit);
        }
    }

    (digits[0], digits[digits.len() - 1])
}

fn create_trie() -> Trie {
    let mut trie = Trie::new();
    trie.insert("one");
    trie.insert("two");
    trie.insert("three");
    trie.insert("four");
    trie.insert("five");
    trie.insert("six");
    trie.insert("seven");
    trie.insert("eight");
    trie.insert("nine");

    return trie;
}

fn str_to_digit(value: &str) -> Option<char> {
    match value {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn parse_line(line: &str) -> String {
    let trie = create_trie();
    let mut new_line = String::new();
    let mut word = String::new();

    for ch in line.chars() {
        word.push(ch);
        if ch.is_digit(10) {
            new_line.push(ch);
            word.clear();
        } else if trie.contains_exact(&word) {
            new_line.push(str_to_digit(&word).unwrap());
            word.clear();
            word.push(ch);
        } else if !trie.contains(&word) {
            for _i in 0..word.len() - 1 {
                word.remove(0);
                if trie.contains(&word) {
                    break;
                }
            }
        }
    }

    new_line
}

fn solve_part_1(input: &str) -> Result<String> {
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = get_digits(line);
        total += (first * 10) + second;
    }

    Ok(total.to_string())
}

fn solve_part_2(input: &str) -> Result<String> {
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = get_digits(&parse_line(line));
        total += (first * 10) + second;
    }

    Ok(total.to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = solve_part_1(&input).context("Solving Year 2023 Day 1")?;
    solution.part2 = solve_part_2(&input).context("Solving Year 2023 Day 2")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() -> Result<()> {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let solution = solve_part_2(&input).context("Parse line test")?;
        assert_eq!(solution, "281");
        Ok(())
    }

    #[test]
    fn parse_line_test() -> Result<()> {
        assert_eq!(get_digits(&parse_line("two1nine")), (2, 9));
        assert_eq!(get_digits(&parse_line("eightwothree")), (8, 3));
        assert_eq!(get_digits(&parse_line("abcone2threexyz")), (1, 3));
        assert_eq!(get_digits(&parse_line("xtwone3four")), (2, 4));
        assert_eq!(get_digits(&parse_line("4nineeightseven2")), (4, 2));
        assert_eq!(get_digits(&parse_line("zoneight234")), (1, 4));
        assert_eq!(get_digits(&parse_line("7pqrstsixteen")), (7, 6));
        Ok(())
    }
}

