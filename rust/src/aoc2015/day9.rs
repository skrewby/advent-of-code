use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, LinkedList};

use crate::solution::Solution;
use anyhow::Result;

fn calculate_routes<'a>(
    cities: BTreeSet<&'a str>,
    connections: HashMap<&str, LinkedList<(&str, u32)>>,
) -> HashMap<u32, Vec<&'a str>> {
    let num_cities = cities.len();
    let mut route_lengths: HashMap<u32, Vec<&str>> = HashMap::new();

    for routes in cities.into_iter().permutations(num_cities) {
        let mut previous_location: Option<&str> = None;
        let mut route_distance = 0;

        for current_location in &routes {
            if let Some(l) = previous_location {
                if current_location != &l {
                    let (_, dist) = connections
                        .get(l)
                        .unwrap()
                        .iter()
                        .find(|&&(loc, _)| loc == *current_location)
                        .unwrap();

                    route_distance += dist;
                }
            }

            previous_location = Some(current_location);
        }

        if !route_lengths.contains_key(&route_distance) {
            route_lengths.insert(route_distance, routes);
        }
    }

    route_lengths
}

fn get_routes(input: &str) -> (BTreeSet<&str>, HashMap<&str, LinkedList<(&str, u32)>>) {
    // All unique cities in the input
    let mut cities = BTreeSet::new();
    // All connections between cities in the input with the distance included
    let mut connections: HashMap<&str, LinkedList<(&str, u32)>> = HashMap::new();

    // Create an Adjancency List using the routes from the input
    for route in input.lines() {
        let route_info: Vec<&str> = route.split(' ').collect();
        let from = route_info[0];
        let to = route_info[2];
        let dist = route_info[4].parse::<u32>().unwrap();

        cities.insert(from);
        cities.insert(to);

        if !connections.contains_key(from) {
            connections.insert(from, LinkedList::new());
        }
        if !connections.contains_key(to) {
            connections.insert(to, LinkedList::new());
        }

        connections.get_mut(from).unwrap().push_back((to, dist));
        connections.get_mut(to).unwrap().push_back((from, dist));
    }

    (cities, connections)
}

fn part_1(routes: &HashMap<u32, Vec<&str>>) -> Result<String> {
    let (minimum_length, _) = routes.iter().min_by_key(|&(len, _)| len).unwrap();

    Ok(minimum_length.to_string())
}

fn part_2(routes: &HashMap<u32, Vec<&str>>) -> Result<String> {
    let (maximum_length, _) = routes.iter().max_by_key(|&(len, _)| len).unwrap();

    Ok(maximum_length.to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let (cities, connections) = get_routes(&input);

    let routes = calculate_routes(cities, connections);

    solution.part1 = part_1(&routes)?;
    solution.part2 = part_2(&routes)?;

    Ok(())
}
