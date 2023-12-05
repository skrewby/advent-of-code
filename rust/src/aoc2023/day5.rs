use crate::solution::Solution;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq)]
enum ParseState {
    SEEDS,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}

fn map_values(map: &mut HashMap<u32, u32>, line: &str) {
    let values: Vec<u32> = line
        .split(' ')
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .collect();

    let dest_start = values[0];
    let source_start = values[1];
    let range = values[2];

    for (i, source) in (source_start..source_start + range).enumerate() {
        let dest = dest_start + i as u32;
        map.insert(source, dest);
    }
}

fn parse_input(
    input: &str,
    soil_map: &mut HashMap<u32, u32>,
    fertilizer_map: &mut HashMap<u32, u32>,
    water_map: &mut HashMap<u32, u32>,
    light_map: &mut HashMap<u32, u32>,
    temp_map: &mut HashMap<u32, u32>,
    humidity_map: &mut HashMap<u32, u32>,
    location_map: &mut HashMap<u32, u32>,
) -> Result<Vec<u32>> {
    let mut seeds: Vec<u32> = vec![0; 0];

    let mut state: ParseState = ParseState::SEEDS;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split(':').collect();
        if line.find(':').is_some() {
            state = match tokens[0] {
                "seeds" => ParseState::SEEDS,
                "seed-to-soil map" => ParseState::SOIL,
                "soil-to-fertilizer map" => ParseState::FERTILIZER,
                "fertilizer-to-water map" => ParseState::WATER,
                "water-to-light map" => ParseState::LIGHT,
                "light-to-temperature map" => ParseState::TEMPERATURE,
                "temperature-to-humidity map" => ParseState::HUMIDITY,
                "humidity-to-location map" => ParseState::LOCATION,
                _ => ParseState::SEEDS,
            };
        } else if !line.is_empty() {
            match state {
                ParseState::SEEDS => {}
                ParseState::SOIL => map_values(soil_map, tokens[0]),
                ParseState::FERTILIZER => map_values(fertilizer_map, tokens[0]),
                ParseState::WATER => map_values(water_map, tokens[0]),
                ParseState::LIGHT => map_values(light_map, tokens[0]),
                ParseState::TEMPERATURE => map_values(temp_map, tokens[0]),
                ParseState::HUMIDITY => map_values(humidity_map, tokens[0]),
                ParseState::LOCATION => map_values(location_map, tokens[0]),
            }
        }
        if state == ParseState::SEEDS {
            seeds = tokens[1]
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| u32::from_str_radix(num.trim(), 10).unwrap())
                .collect();
        }
    }

    Ok(seeds)
}

fn get_map_value(seed: u32, map: &HashMap<u32, u32>) -> u32 {
    match map.contains_key(&seed) {
        true => map[&seed],
        false => seed,
    }
}

fn get_value(
    seed: u32,
    soil_map: &HashMap<u32, u32>,
    fertilizer_map: &HashMap<u32, u32>,
    water_map: &HashMap<u32, u32>,
    light_map: &HashMap<u32, u32>,
    temp_map: &HashMap<u32, u32>,
    humidity_map: &HashMap<u32, u32>,
    location_map: &HashMap<u32, u32>,
    value_wanted: ParseState,
) -> u32 {
    match value_wanted {
        ParseState::SEEDS => seed,
        ParseState::SOIL => get_map_value(seed, soil_map),
        ParseState::FERTILIZER => get_map_value(get_map_value(seed, soil_map), fertilizer_map),
        ParseState::WATER => get_map_value(
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::FERTILIZER,
            ),
            water_map,
        ),
        ParseState::LIGHT => get_map_value(
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::WATER,
            ),
            light_map,
        ),
        ParseState::TEMPERATURE => get_map_value(
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::LIGHT,
            ),
            temp_map,
        ),
        ParseState::HUMIDITY => get_map_value(
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::TEMPERATURE,
            ),
            humidity_map,
        ),
        ParseState::LOCATION => get_map_value(
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::HUMIDITY,
            ),
            location_map,
        ),
    }
}

fn get_min_location(
    seeds: Vec<u32>,
    soil_map: &HashMap<u32, u32>,
    fertilizer_map: &HashMap<u32, u32>,
    water_map: &HashMap<u32, u32>,
    light_map: &HashMap<u32, u32>,
    temp_map: &HashMap<u32, u32>,
    humidity_map: &HashMap<u32, u32>,
    location_map: &HashMap<u32, u32>,
) -> u32 {
    seeds
        .par_iter()
        .map(|&seed| {
            get_value(
                seed,
                soil_map,
                fertilizer_map,
                water_map,
                light_map,
                temp_map,
                humidity_map,
                location_map,
                ParseState::LOCATION,
            )
        })
        .min()
        .unwrap()
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let mut soil_map: HashMap<u32, u32> = HashMap::new();
    let mut fertilizer_map: HashMap<u32, u32> = HashMap::new();
    let mut water_map: HashMap<u32, u32> = HashMap::new();
    let mut light_map: HashMap<u32, u32> = HashMap::new();
    let mut temp_map: HashMap<u32, u32> = HashMap::new();
    let mut humidity_map: HashMap<u32, u32> = HashMap::new();
    let mut location_map: HashMap<u32, u32> = HashMap::new();
    let seeds = parse_input(
        &input,
        &mut soil_map,
        &mut fertilizer_map,
        &mut water_map,
        &mut light_map,
        &mut temp_map,
        &mut humidity_map,
        &mut location_map,
    )?;

    let closest_loc = get_min_location(
        seeds,
        &soil_map,
        &fertilizer_map,
        &water_map,
        &light_map,
        &temp_map,
        &humidity_map,
        &location_map,
    );
    solution.part1 = closest_loc.to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parse_test() -> Result<()> {
        let mut soil_map: HashMap<u32, u32> = HashMap::new();
        let mut fertilizer_map: HashMap<u32, u32> = HashMap::new();
        let mut water_map: HashMap<u32, u32> = HashMap::new();
        let mut light_map: HashMap<u32, u32> = HashMap::new();
        let mut temp_map: HashMap<u32, u32> = HashMap::new();
        let mut humidity_map: HashMap<u32, u32> = HashMap::new();
        let mut location_map: HashMap<u32, u32> = HashMap::new();
        let seeds = parse_input(
            INPUT,
            &mut soil_map,
            &mut fertilizer_map,
            &mut water_map,
            &mut light_map,
            &mut temp_map,
            &mut humidity_map,
            &mut location_map,
        )?;

        assert_eq!(get_map_value(98, &soil_map), 50);
        assert_eq!(get_map_value(99, &soil_map), 51);
        assert_eq!(get_map_value(48, &soil_map), 48);
        assert_eq!(get_map_value(49, &soil_map), 49);
        assert_eq!(get_map_value(50, &soil_map), 52);
        assert_eq!(get_map_value(79, &soil_map), 81);
        assert_eq!(get_map_value(14, &soil_map), 14);
        assert_eq!(get_map_value(55, &soil_map), 57);
        assert_eq!(get_map_value(13, &soil_map), 13);

        assert_eq!(
            get_value(
                79,
                &soil_map,
                &fertilizer_map,
                &water_map,
                &light_map,
                &temp_map,
                &humidity_map,
                &location_map,
                ParseState::LOCATION
            ),
            82
        );
        assert_eq!(
            get_value(
                14,
                &soil_map,
                &fertilizer_map,
                &water_map,
                &light_map,
                &temp_map,
                &humidity_map,
                &location_map,
                ParseState::LOCATION
            ),
            43
        );
        assert_eq!(
            get_value(
                55,
                &soil_map,
                &fertilizer_map,
                &water_map,
                &light_map,
                &temp_map,
                &humidity_map,
                &location_map,
                ParseState::LOCATION
            ),
            86
        );
        assert_eq!(
            get_value(
                13,
                &soil_map,
                &fertilizer_map,
                &water_map,
                &light_map,
                &temp_map,
                &humidity_map,
                &location_map,
                ParseState::LOCATION
            ),
            35
        );

        let min_location = get_min_location(
            seeds,
            &soil_map,
            &fertilizer_map,
            &water_map,
            &light_map,
            &temp_map,
            &humidity_map,
            &location_map,
        );
        assert_eq!(min_location, 35);
        Ok(())
    }

    #[test]
    fn tempfn() -> Result<()> {
        Ok(())
    }
}
