use crate::solution::Solution;
use anyhow::Result;

enum State {
    Normal,
    Escape,
    Hex,
}

fn encode(text: &str) -> String {
    let mut encoded = "\"".to_owned();

    let mut state = State::Normal;
    for c in text.chars() {
        match state {
            State::Normal => match c {
                '"' => {
                    encoded.push('\\');
                    encoded.push('"');
                }
                '\\' => {
                    encoded.push('\\');
                    encoded.push('\\');
                    state = State::Escape;
                }
                _ => encoded.push(c),
            },
            State::Escape => match c {
                '"' => {
                    encoded.push('\\');
                    encoded.push('"');
                    state = State::Normal;
                }
                '\\' => {
                    encoded.push('\\');
                    encoded.push('\\');
                    state = State::Normal;
                }
                _ => {
                    encoded.push(c);
                    state = State::Normal;
                }
            },
            _ => {}
        }
    }

    encoded.push('"');
    encoded
}

fn string_len(text: &mut String) -> i32 {
    text.pop();
    text.remove(0);

    let mut count = 0;
    let mut state = State::Normal;
    for c in text.chars() {
        match state {
            State::Normal => {
                if c == '\\' {
                    state = State::Escape;
                } else {
                    count += 1;
                }
            }
            State::Escape => match c {
                '\\' => {
                    count += 1;
                    state = State::Normal;
                }
                '"' => {
                    count += 1;
                    state = State::Normal;
                }
                'x' => state = State::Hex,
                _ => {}
            },
            State::Hex => {
                state = State::Normal;
            }
        }
    }

    count
}

fn part_1(input: &str) -> Result<String> {
    let mut byte_length = 0;
    let mut str_length = 0;
    for text in input.lines() {
        byte_length += text.len();
        str_length += string_len(&mut text.to_owned());
    }

    let diff = byte_length - str_length as usize;
    Ok(diff.to_string())
}

fn part_2(input: &str) -> Result<String> {
    let mut prev_len = 0;
    let mut after_len = 0;
    for text in input.lines() {
        prev_len += text.len();
        let encoded_str = encode(text);
        after_len += encoded_str.len();
    }

    let diff = after_len - prev_len;
    Ok(diff.to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input)?;
    solution.part2 = part_2(&input)?;

    Ok(())
}
