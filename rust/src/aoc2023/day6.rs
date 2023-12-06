use crate::solution::Solution;
use anyhow::Result;
use itertools::Itertools;

struct Race {
    time: u64,
    distance: u64,
}

// All the times that the button can be held to beat the record
fn record_strategies(time: u64, distance: u64) -> Vec<u64> {
    (0..time + 1)
        .into_iter()
        .filter(|held_time| {
            let distance_travelled = held_time * (time - held_time);
            distance_travelled > distance
        })
        .collect()
}

fn get_races(input: &str) -> Vec<Race> {
    let mut races = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let time_tokens: Vec<&str> = lines[0].split(':').collect();
    let distance_tokens: Vec<&str> = lines[1].split(':').collect();
    let times: Vec<u64> = time_tokens[1]
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| u64::from_str_radix(num, 10).unwrap())
        .collect();
    let distances: Vec<u64> = distance_tokens[1]
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| u64::from_str_radix(num, 10).unwrap())
        .collect();

    for (i, &time) in times.iter().enumerate() {
        let distance = distances[i];
        races.push(Race { time, distance });
    }

    races
}

fn get_race(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    let time_tokens: Vec<&str> = lines[0].split(':').collect();
    let distance_tokens: Vec<&str> = lines[1].split(':').collect();
    let time: u64 = u64::from_str_radix(
        &time_tokens[1]
            .split(' ')
            .filter(|num| !num.is_empty())
            .join(""),
        10,
    )
    .unwrap();
    let distance: u64 = u64::from_str_radix(
        &distance_tokens[1]
            .split(' ')
            .filter(|num| !num.is_empty())
            .join(""),
        10,
    )
    .unwrap();

    Race { time, distance }
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let races = get_races(&input);
    let mut num_strats_multi = 1;
    for race in races {
        num_strats_multi *= record_strategies(race.time, race.distance).len();
    }
    solution.part1 = num_strats_multi.to_string();

    let race = get_race(&input);
    let strats = record_strategies(race.time, race.distance);
    solution.part2 = strats.len().to_string();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn record_strategies_test() -> Result<()> {
        let strats = record_strategies(7, 9);
        assert_eq!(strats.len(), 4);

        let strats = record_strategies(15, 40);
        assert_eq!(strats.len(), 8);

        let strats = record_strategies(30, 200);
        assert_eq!(strats.len(), 9);

        Ok(())
    }
    #[test]
    fn race_strategies_test() -> Result<()> {
        let race = get_race(&INPUT);
        let strats = record_strategies(race.time, race.distance);
        assert_eq!(strats.len(), 71503);
        Ok(())
    }
}
