use std::collections::LinkedList;

use crate::solution::Solution;
use anyhow::{Context, Result};

struct Card {
    id: u32,
    win_count: u32,
}

fn parse_card(line: &str) -> Result<Card> {
    let tokens: Vec<&str> = line.split(':').collect();
    let prefix: Vec<&str> = tokens[0].split(' ').filter(|num| !num.is_empty()).collect();
    let postfix: Vec<&str> = tokens[1].split('|').collect();

    let id: u32 = u32::from_str_radix(prefix[1].trim(), 10).context("Parsing Card ID")?;
    let numbers: LinkedList<u32> = postfix[1]
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect();
    let winning_numbers: LinkedList<u32> = postfix[0]
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect();
    let win_count: u32 = numbers
        .iter()
        .filter(|num| winning_numbers.contains(&num))
        .count()
        .try_into()?;

    Ok(Card { id, win_count })
}

fn parse_input(input: &str) -> Result<Vec<Card>> {
    let mut cards: Vec<Card> = vec![];
    for line in input.lines() {
        cards.push(parse_card(line)?);
    }

    Ok(cards)
}

fn copy_cards(cards: &Vec<Card>) -> Result<Vec<u32>> {
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    for card in cards {
        for i in 1..card.win_count + 1 {
            card_counts[card.id as usize + i as usize - 1] += 1 * card_counts[card.id as usize - 1];
        }
    }

    Ok(card_counts)
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let cards = parse_input(&input)?;
    let sum_points: u32 = cards
        .iter()
        .filter(|card| card.win_count > 0)
        .map(|card| u32::pow(2, card.win_count - 1))
        .sum();
    let copies = copy_cards(&cards)?;
    let total_copies: u32 = copies.iter().sum();

    solution.part1 = sum_points.to_string();
    solution.part2 = total_copies.to_string();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_points_test() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parse_input(input)?;
        let sum_points: u32 = cards
            .iter()
            .filter(|card| card.win_count > 0)
            .map(|card| u32::pow(2, card.win_count - 1))
            .sum();
        assert_eq!(sum_points, 13);

        Ok(())
    }

    #[test]
    fn sum_cards_test() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parse_input(input)?;
        let copies = copy_cards(&cards)?;

        let total_copies: u32 = copies.iter().sum();
        assert_eq!(total_copies, 30);

        Ok(())
    }
}
