use std::collections::VecDeque;

use crate::solution::Solution;
use anyhow::Result;

fn create_sequence(input: String) -> String {
    let mut seq = String::new();
    let mut q: VecDeque<char> = input.chars().collect();

    let mut count = 0;
    let mut prev_ch = None;
    let mut ch = ' ';
    while !q.is_empty() {
        ch = q.pop_front().unwrap();
        if let Some(prev) = prev_ch {
            if prev != ch {
                seq.push_str(count.to_string().as_str());
                seq.push(prev);
                count = 0;
            }
        }
        count += 1;
        prev_ch = Some(ch);
    }
    seq.push_str(count.to_string().as_str());
    seq.push(ch);

    seq
}

fn get_sequence(input: &str, n: u32) -> String {
    let mut seq: String = create_sequence(input.to_owned());

    for _ in 1..n {
        seq = create_sequence(seq);
    }

    seq
}

fn part_1(input: &str) -> Result<String> {
    Ok(get_sequence(input, 40).len().to_string())
}

fn part_2(input: &str) -> Result<String> {
    Ok(get_sequence(input, 50).len().to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input)?;
    solution.part2 = part_2(&input)?;

    Ok(())
}
