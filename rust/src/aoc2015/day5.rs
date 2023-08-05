use crate::solution::Solution;
use anyhow::Result;
use std::collections::HashSet;

fn contains_three_vowels(input: &str) -> bool {
    let mut vowel_count = 0;
    let vowels = HashSet::from(['a', 'i', 'e', 'o', 'u']);

    for ch in input.chars() {
        if vowels.contains(&ch) {
            vowel_count += 1;
        }
    }

    vowel_count >= 3
}

fn contains_double_letter(input: &str) -> bool {
    let mut prev_ch = ' ';
    for ch in input.chars() {
        if prev_ch == ch {
            return true;
        }

        prev_ch = ch;
    }

    false
}

fn contains_double_letter_ext(input: &str) -> bool {
    for (i, ch) in input.char_indices().skip(1) {
        let mut pair = input[i - 1..i].to_string();
        pair.push_str(&ch.to_string());

        if input[i + 1..].contains(&pair) {
            return true;
        }
    }

    false
}

fn contains_forbidden_strings(input: &str) -> bool {
    let blacklist = HashSet::from([
        "ab".to_string(),
        "cd".to_string(),
        "pq".to_string(),
        "xy".to_string(),
    ]);

    let mut prev_ch = ' ';
    for ch in input.chars() {
        let mut pair = prev_ch.to_string();
        pair.push_str(&ch.to_string());
        if blacklist.contains(&pair) {
            return true;
        }

        prev_ch = ch;
    }

    false
}

fn contains_repeat_pattern(input: &str) -> bool {
    input
        .char_indices()
        .skip(2)
        .any(|(i, ch)| input[i - 2..i - 1] == ch.to_string())
}

fn is_string_nice(input: &str) -> bool {
    contains_three_vowels(input)
        && contains_double_letter(input)
        && !contains_forbidden_strings(input)
}

fn is_string_nice_ext(input: &str) -> bool {
    contains_double_letter_ext(input) && contains_repeat_pattern(input)
}

fn num_valid_string(input: &str) -> i32 {
    let mut count = 0;

    for line in input.split('\n') {
        if is_string_nice(line) {
            count += 1;
        }
    }

    count
}

fn num_valid_string_ext(input: &str) -> i32 {
    let mut count = 0;

    for line in input.split('\n') {
        if is_string_nice_ext(line) {
            count += 1;
        }
    }

    count
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = num_valid_string(&input).to_string();
    solution.part2 = num_valid_string_ext(&input).to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_double_letters_ext() {
        assert!(contains_double_letter_ext("kimmalsummwqp"));
        assert!(!contains_double_letter_ext("sduiwbfoqwkc"));
        assert!(!contains_double_letter_ext("aaa"));
        assert!(contains_double_letter_ext("fuwiasmcpquasbv"));
    }

    #[test]
    fn test_contains_repeated_patterns() {
        assert!(contains_repeat_pattern("kimxmalsummwqp"));
        assert!(!contains_repeat_pattern("sduiwbfoqwkc"));
    }

    #[test]
    fn test_is_string_nice() {
        assert!(is_string_nice("ugknbfddgicrmopn"));
        assert!(is_string_nice("aaa"));
        assert!(!is_string_nice("jchzalrnumimnmhp"));
        assert!(!is_string_nice("haegwjzuvuyypxyu"));
        assert!(!is_string_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_string_nice_ext() {
        assert!(is_string_nice_ext("qjhvhtzxzqqjkmpb"));
        assert!(is_string_nice_ext("xxyxx"));
        assert!(!is_string_nice_ext("uurcxstgmygtbstg"));
        assert!(!is_string_nice_ext("ieodomkazucvgmuy"));
    }
}
