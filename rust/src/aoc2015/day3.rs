use crate::solution::Solution;
use std::collections::HashMap;
use anyhow::Result;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct House {
    x: i32,
    y: i32,
}

impl House {
    fn new() -> House {
        House {
            x: 0,
            y: 0
        }
    }

    fn travel(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn direction_moved(command: char) -> (i32, i32) {
    match command {
        '^' => (0, 1),
        '<' => (-1, 0),
        'v' => (0, -1),
        '>' => (1, 0),
        _ => (0, 0),
    }
}

fn houses_visited(commands: &str) -> HashMap<House, bool> {
    let mut visited = HashMap::new();
    let mut current_house = House::new();
    visited.insert(current_house, true);
    for cmd in commands.chars() {
        let (dx, dy) = direction_moved(cmd);
        current_house.travel(dx, dy);
        visited.insert(current_house, true);
    }

    visited
}

fn houses_visited_robo(commands: &str) -> HashMap<House, bool> {
    let santa_commands: String = commands.chars().step_by(2).collect();
    let robo_commands: String = commands.chars().skip(1).step_by(2).collect();

    let mut santa_houses = houses_visited(&santa_commands);
    let robo_houses = houses_visited(&robo_commands);
    santa_houses.extend(robo_houses.iter());

    santa_houses
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = houses_visited(&input).len().to_string();
    solution.part2 = houses_visited_robo(&input).len().to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_houses_visited() {
        assert_eq!(houses_visited(">").len(), 2);
        assert_eq!(houses_visited("^>v<").len(), 4);
        assert_eq!(houses_visited("^v^v^v^v^v").len(), 2);
    }

    #[test]
    fn test_houses_visited_robbo() {
        assert_eq!(houses_visited_robo("^v").len(), 3);
        assert_eq!(houses_visited_robo("^>v<").len(), 3);
        assert_eq!(houses_visited_robo("^v^v^v^v^v").len(), 11);
    }

    #[test]
    fn test_direction_moved() {
        assert_eq!(direction_moved('^'), (0, 1));
        assert_eq!(direction_moved('<'), (-1, 0));
        assert_eq!(direction_moved('v'), (0, -1));
        assert_eq!(direction_moved('>'), (1, 0));
    }
}
