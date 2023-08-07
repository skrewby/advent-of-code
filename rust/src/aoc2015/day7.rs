use crate::solution::Solution;
use anyhow::{Context, Result};
use std::collections::HashMap;

fn get_args(arg1: &str, arg2: &str, reg: &mut HashMap<String, u16>) -> (u16, u16) {
    let left_arg = match arg1.parse::<u16>() {
        Ok(i) => i,
        Err(_) => reg[arg1]
    };
    let right_arg = match arg2.parse::<u16>() {
        Ok(i) => i,
        Err(_) => reg[arg2]
    };

    (left_arg, right_arg)
}

fn mv(src: &str, dest: &str, reg: &mut HashMap<String, u16>) -> Result<()> {
    let result = src.parse::<u16>();

    match result {
        Ok(i) => reg.insert(dest.to_owned(), i),
        Err(_) => reg.insert(dest.to_owned(), reg[src]),
    };

    Ok(())
}

fn op_and(arg1: &str, arg2: &str, dest: String, reg: &mut HashMap<String, u16>) {
    let (left_arg, right_arg) = get_args(arg1, arg2, reg);
    reg.insert(dest, left_arg & right_arg);
}

fn op_or(arg1: &str, arg2: &str, dest: String, reg: &mut HashMap<String, u16>) {
    let (left_arg, right_arg) = get_args(arg1, arg2, reg);
    reg.insert(dest, left_arg | right_arg);
}

fn op_lshift(arg1: &str, arg2: &str, dest: String, reg: &mut HashMap<String, u16>) {
    let (left_arg, right_arg) = get_args(arg1, arg2, reg);
    reg.insert(dest, left_arg << right_arg);
}

fn op_rshift(arg1: &str, arg2: &str, dest: String, reg: &mut HashMap<String, u16>) {
    let (left_arg, right_arg) = get_args(arg1, arg2, reg);
    reg.insert(dest, left_arg >> right_arg);
}

fn bitwise(
    arg1: &str,
    arg2: &str,
    op: &str,
    dest: String,
    reg: &mut HashMap<String, u16>,
) -> Result<()> {
     match op {
        "AND" => op_and(arg1, arg2, dest, reg), 
        "OR" => op_or(arg1, arg2, dest, reg), 
        "LSHIFT" => op_lshift(arg1, arg2, dest, reg), 
        "RSHIFT" => op_rshift(arg1, arg2, dest, reg), 
        _ => (),
    };

    Ok(())
}

fn not(src: &str, dest: String, reg: &mut HashMap<String, u16>) -> Result<()> {
    reg.insert(dest, !reg[src]);

    Ok(())
}

#[allow(dead_code)]
fn run_cmd(cmd: &str, reg: &mut HashMap<String, u16>) -> Result<()> {
    let tokens: Vec<String> = cmd.split(' ').map(|x| x.to_string()).collect();

    match tokens.len() {
        3 => mv(&tokens[0], &tokens[2], reg)?,
        4 => not(&tokens[1], tokens[3].clone(), reg)?,
        5 => bitwise(&tokens[0], &tokens[2], &tokens[1], tokens[4].clone(), reg)?,
        _ => {}
    };

    Ok(())
}

fn run(tokens: &Vec<String>, reg: &mut HashMap<String, u16>) -> Result<()> {
    match tokens.len() {
        3 => mv(&tokens[0], &tokens[2], reg)?,
        4 => not(&tokens[1], tokens[3].clone(), reg)?,
        5 => bitwise(&tokens[0], &tokens[2], &tokens[1], tokens[4].clone(), reg)?,
        _ => {}
    };

    Ok(())
}

fn is_command_valid(tokens: &[String], reg: &HashMap<String, u16>) -> bool {
    match tokens.len() {
        3 => {
            let arg1_num = &tokens[0].parse::<u16>();
            arg1_num.is_ok() || reg.contains_key(&tokens[0])
        },
        4 => reg.contains_key(&tokens[1]),
        5 => {
            let arg1 = &tokens[0];
            let arg2 = &tokens[2];

            let arg1_is_num = arg1.parse::<u16>();
            let arg2_is_num = arg2.parse::<u16>();

            (arg1_is_num.is_ok() || reg.contains_key(arg1))
                && (arg2_is_num.is_ok() || reg.contains_key(arg2))
        }
        _ => false,
    }
}

// Will search for a valid instruction and execute it until all instructions have been executed
fn find_and_execute(
    commands: &mut HashMap<usize, Vec<String>>,
    reg: &mut HashMap<String, u16>,
) -> Result<()> {
    if commands.is_empty() {
        return Ok(());
    }

    let mut executed_commands: Vec<usize> = Vec::new();
    for cmd in commands.iter() {
        let tokens = cmd.1;

        if is_command_valid(tokens, reg) {
            run(tokens, reg)?;
            executed_commands.push(*cmd.0);
        }
    }

    for i in executed_commands {
        commands.remove(&i);
    }

    find_and_execute(commands, reg)
}

fn build_circuit(input: &str) -> Result<HashMap<String, u16>> {
    let mut reg = HashMap::new();

    // Add all commands to a map so we can remove them as we execute the valid ones
    let mut cmds = HashMap::new();
    for (i, cmd) in input.lines().enumerate() {
        let tokens: Vec<String> = cmd.split(' ').map(|x| x.to_string()).collect();
        cmds.insert(i, tokens);
    }

    find_and_execute(&mut cmds, &mut reg)?;

    Ok(reg)
}

fn part_1(input: &str) -> Result<String> {
    let reg = build_circuit(input)?;

    Ok(reg["a"].to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input).context("Completing Year 2015 Day 7 Part 1")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> Result<()> {
        let mut reg: HashMap<String, u16> = HashMap::new();
        run_cmd("123 -> x", &mut reg)?;
        assert_eq!(reg["x"], 123);
        run_cmd("456 -> y", &mut reg)?;
        assert_eq!(reg["y"], 456);
        run_cmd("x AND y -> d", &mut reg)?;
        assert_eq!(reg["d"], reg["x"] & reg["y"]);
        run_cmd("x OR y -> e", &mut reg)?;
        assert_eq!(reg["e"], reg["x"] | reg["y"]);
        run_cmd("x LSHIFT 2 -> f", &mut reg)?;
        assert_eq!(reg["f"], reg["x"] << 2);
        run_cmd("y RSHIFT 2 -> g", &mut reg)?;
        assert_eq!(reg["g"], reg["y"] >> 2);
        run_cmd("NOT x -> h", &mut reg)?;
        assert_eq!(reg["h"], !reg["x"]);
        run_cmd("NOT y -> i", &mut reg)?;
        assert_eq!(reg["i"], !reg["y"]);

        assert_eq!(reg["d"], 72);
        assert_eq!(reg["e"], 507);
        assert_eq!(reg["f"], 492);
        assert_eq!(reg["g"], 114);
        assert_eq!(reg["h"], 65412);
        assert_eq!(reg["i"], 65079);
        assert_eq!(reg["x"], 123);
        assert_eq!(reg["y"], 456);

        Ok(())
    }
}
