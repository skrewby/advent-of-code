use std::collections::HashSet;

use crate::solution::Solution;
use anyhow::Result;

fn increment(pass: &str) -> String {
    let mut arr: Vec<char> = pass.chars().collect();

    let mut carry = true;
    let mut n = arr.len() - 1;
    while carry {
        let mut ch = arr[n] as u32;
        ch += 1;

        if ch == 123 {
            ch = 97;
            if n == 0 {
                carry = false;
            }
        } else {
            carry = false;
        }

        arr[n] = char::from_u32(ch).unwrap();

        if carry {
            n -= 1;
        }
    }

    arr.into_iter().collect()
}

fn has_straight_seq(input: &str) -> bool {
    let arr: Vec<char> = input.chars().collect();

    for i in 2..arr.len() {
        if arr[i - 2] as u32 + 1 == arr[i - 1] as u32 && arr[i - 1] as u32 + 1 == arr[i] as u32 {
            return true;
        }
    }

    false
}

fn is_blacklisted(input: &str, blacklist: &Vec<char>) -> bool {
    for ch in blacklist {
        if input.contains(*ch) {
            return true;
        }
    }

    false
}

fn has_overlaps(input: &str) -> bool {
    let mut overlaps: HashSet<char> = HashSet::new();
    let arr: Vec<char> = input.chars().collect();

    for i in 1..arr.len() {
        if arr[i - 1] == arr[i] {
            overlaps.insert(arr[i]);
        }
    }

    overlaps.len() > 1
}

fn part_1(input: &str) -> Result<String> {
    let mut pass = input.clone().to_string();
    pass = increment(&pass);

    while !has_straight_seq(&pass)
        || !has_overlaps(&pass)
        || is_blacklisted(&pass, &vec!['i', 'o', 'l'])
    {
        pass = increment(&pass);
    }

    Ok(pass)
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input)?;
    solution.part2 = part_1(&solution.part1)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() -> Result<()> {
        assert_eq!(increment("aa"), "ab");
        assert_eq!(increment("xx"), "xy");
        assert_eq!(increment("xz"), "ya");
        assert_eq!(increment("ya"), "yb");

        Ok(())
    }

    #[test]
    fn test_straight() -> Result<()> {
        assert!(has_straight_seq("hijklmmn"));
        assert!(!has_straight_seq("abbceffg"));
        assert!(!has_straight_seq("abbcegjk"));
        assert!(has_straight_seq("kdnxyzld"));

        Ok(())
    }

    #[test]
    fn test_blacklist() -> Result<()> {
        assert!(is_blacklisted("hijklmmn", &vec!['i', 'o', 'l']));
        assert!(!is_blacklisted("abbcegjk", &vec!['i', 'o', 'l']));
        Ok(())
    }

    #[test]
    fn test_overlaps() -> Result<()> {
        assert!(!has_overlaps("hijklmmn"));
        assert!(has_overlaps("abbceffg"));

        Ok(())
    }
}
