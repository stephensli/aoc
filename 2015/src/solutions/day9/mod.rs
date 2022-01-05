use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use helpers::{ChallengeSolution, Solution};

use itertools::Itertools;

pub(crate) struct Challenge {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TravelNode {
    pub source: String,
    pub target: String,
    pub cost: u32,
}

/// Takes in a line based on the given format and converts it into a given target
/// and source destination, the target contains the cost of travel.
///
/// # Arguments
///
/// * `line`:
///
/// returns: (String, Target)
fn parse_line(line: &str) -> TravelNode {
    let line_split = line.split_whitespace().into_iter().collect::<Vec<&str>>();

    TravelNode {
        source: line_split[0].to_string(),
        target: line_split[2].to_string(),
        cost: line_split[4].parse::<u32>().unwrap(),
    }
}


fn determine_distance_values(places: HashSet<String>, distances: HashMap<String, HashMap<String, u32>>) -> (u32, u32) {
    let mut shortest_value = u32::MAX;
    let mut longest_value = u32::MIN;

    // generate all possible permutations, these permutations are every possible
    // route that can be taken from A to B. These will be used to determine costs.
    for x in places.iter().permutations(places.len()).unique() {
        // slice the first half and second half of the paths taken. Each
        // setup will be summed up until the destination is hit.
        //
        // e.g
        //
        // 0->1 - SUM
        // 1->2 - SUM
        //
        // etc
        let first_half = x[0..x.len() - 1].to_vec();
        let second_half = x[1..x.len()].to_vec();

        let mut sum_distances = 0;

        // iterate and sum the distances.
        for i in 0..first_half.len() {
            sum_distances += distances.get(first_half[i]).unwrap().get(second_half[i]).unwrap();
        }

        // check if we have a new min or max values.
        shortest_value = min(shortest_value, sum_distances);
        longest_value = max(longest_value, sum_distances);
    }

    (shortest_value, longest_value)
}

impl ChallengeSolution for Challenge {
    fn solve(&self, input: Vec<String>) -> Solution {
        // We must go from start and end at any two (different) locations, but
        // he must visit each location exactly once. This is a unique listing
        // of all the places.
        let mut places: HashSet<String> = HashSet::new();

        // a mapping between every single place and the cost to get to said
        // place between those locations. E.g the cost between X => Y.
        let mut destinations: HashMap<String, HashMap<String, u32>> = HashMap::new();

        for x in input {
            let parsed_line = &parse_line(&x);

            places.insert(parsed_line.source.clone());
            places.insert(parsed_line.target.clone());

            let source_entry = destinations
                .entry(parsed_line.source.clone())
                .or_insert(HashMap::new());

            source_entry.insert(parsed_line.target.clone(), parsed_line.cost);

            let destination_entry = destinations
                .entry(parsed_line.target.clone())
                .or_insert(HashMap::new());

            destination_entry.insert(parsed_line.source.clone(), parsed_line.cost);
        }

       let distances = determine_distance_values(places, destinations);

        Solution {
            part1: distances.0.to_string(),
            part2: distances.1.to_string(),
        }
    }

    fn year(&self) -> i32 {
        2015
    }

    fn day(&self) -> i32 {
        9
    }

    fn example(&self) -> bool {
        false
    }
}
