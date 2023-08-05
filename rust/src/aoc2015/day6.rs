use crate::solution::Solution;
use anyhow::Result;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    fn location(&self) -> usize {
        self.y * HEIGHT + self.x
    }
}

fn parse_locations(parameters: &[&str], coord_start: usize) -> Vec<Coord> {
    let top: Vec<&str> = parameters[coord_start].split(',').collect();
    let bot: Vec<&str> = parameters[coord_start + 2].split(',').collect();

    let top_left = Coord::new(
        top[0].parse::<usize>().unwrap(),
        top[1].parse::<usize>().unwrap(),
    );
    let bottom_right = Coord::new(
        bot[0].parse::<usize>().unwrap(),
        bot[1].parse::<usize>().unwrap(),
    );
    let width = bottom_right.x - top_left.x;
    let height = bottom_right.y - top_left.y;

    let mut houses: Vec<Coord> = Vec::new();
    for y in top_left.y..top_left.y + height + 1 {
        for x in top_left.x..top_left.x + width + 1 {
            houses.push(Coord::new(x, y));
        }
    }

    houses
}

fn parse_commands(map: &mut [bool; WIDTH * HEIGHT], commands: &str) {
    for line in commands.lines() {
        let parameters: Vec<&str> = line.split(' ').collect();
        let (command, coord_start) = match parameters[0] {
            "turn" => (parameters[1], 2),
            "toggle" => ("toggle", 1),
            _ => panic!("Couldn't parse commands"),
        };
        let locations = parse_locations(&parameters, coord_start);

        for house in locations {
            match command {
                "on" => map[house.location()] = true,
                "off" => map[house.location()] = false,
                "toggle" => map[house.location()] ^= true,
                _ => {}
            }
        }
    }
}

fn parse_commands_brightness(map: &mut [u32; WIDTH * HEIGHT], commands: &str) {
    for line in commands.lines() {
        let parameters: Vec<&str> = line.split(' ').collect();
        let (command, coord_start) = match parameters[0] {
            "turn" => (parameters[1], 2),
            "toggle" => ("toggle", 1),
            _ => panic!("Couldn't parse commands"),
        };
        let locations = parse_locations(&parameters, coord_start);

        for house in locations {
            match command {
                "on" => map[house.location()] += 1,
                "off" => {
                    if map[house.location()] > 0 {
                        map[house.location()] -= 1;
                    }
                }
                "toggle" => map[house.location()] += 2,
                _ => {}
            }
        }
    }
}

fn part_1(input: &str) -> String {
    let mut map = [false; WIDTH * HEIGHT];
    parse_commands(&mut map, input);

    let count = map.iter().filter(|&&x| x).count();
    count.to_string()
}

fn part_2(input: &str) -> String {
    let mut map = [0; WIDTH * HEIGHT];
    parse_commands_brightness(&mut map, input);

    let count: u32 = map.iter().sum();
    count.to_string()
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = part_1(&input);
    solution.part2 = part_2(&input);

    Ok(())
}
