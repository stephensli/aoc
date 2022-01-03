use std::collections::HashMap;

use helpers::{ChallengeSolution, Solution};

#[cfg(test)]
mod test;

pub(crate) struct Challenge {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TravelNode {
    pub source: String,
    pub target: String,
    pub cost: i32,
}

/// Takes in a line based on the given format and converts it into a given target
/// and source destination, the target contains the cost of travel.
///
/// # Arguments
///
/// * `line`:
///
/// returns: (String, Target)
fn parse_line(line: &str) -> (TravelNode, TravelNode) {
    let line_split = line.split_whitespace().into_iter().collect::<Vec<&str>>();
    (
        TravelNode {
            source: line_split[line_split.len() - 3].to_string(),
            target: line_split[0].to_string(),
            cost: line_split[line_split.len() - 1].parse::<i32>().unwrap(),
        },
        TravelNode {
            source: line_split[0].to_string(),
            cost: line_split[line_split.len() - 1].parse::<i32>().unwrap(),
            target: line_split[line_split.len() - 3].to_string(),
        },
    )
}

fn determine_shortest_distance_visit_once(input: HashMap<String, Vec<TravelNode>>) -> u32 {
    // example process - visit at most once.
    //
    // Dublin -> London -> Belfast = 982
    // London -> Dublin -> Belfast = 605
    // London -> Belfast -> Dublin = 659
    // Dublin -> Belfast -> London = 659
    // Belfast -> Dublin -> London = 605
    // Belfast -> London -> Dublin = 982
    // output: 605

    0
}

impl ChallengeSolution for Challenge {
    fn solve(&self, input: Vec<String>) -> Solution {
        // input sample
        // London to Dublin = 464
        // London to Belfast = 518
        // Dublin to Belfast = 141
        // generate a map from all possible positions and the target cost.
        let mut travel_map: HashMap<String, Vec<TravelNode>> = HashMap::new();

        for x in input {
            let parsed_line = parse_line(&x);

            let first_entry = travel_map
                .entry(parsed_line.0.source.clone())
                .or_insert_with(Vec::new);

            first_entry.push(parsed_line.0);

            let second_entry = travel_map
                .entry(parsed_line.1.source.clone())
                .or_insert_with(Vec::new);

            second_entry.push(parsed_line.1);
        }

        println!("{:?}", travel_map);

        Solution {
            part1: determine_shortest_distance_visit_once(travel_map).to_string(),
            part2: "".to_string(),
        }
    }

    fn year(&self) -> i32 {
        2015
    }

    fn day(&self) -> i32 {
        9
    }

    fn example(&self) -> bool {
        true
    }
}
