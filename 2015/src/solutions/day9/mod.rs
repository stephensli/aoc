use helpers::{ChallengeSolution, Solution};

#[cfg(test)]
mod test;

pub(crate) struct Challenge {}

impl ChallengeSolution for Challenge {
    fn solve(&self, _input: Vec<String>) -> Solution {
        Solution {
            part1: "".to_string(),
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
