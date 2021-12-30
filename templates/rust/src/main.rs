use helpers::{setup, Solution};

pub fn solve(input: String) -> Solution {
    println!("{}", input);

    Solution {
        part1: "".to_string(),
        part2: "".to_string(),
    }
}

fn main() {
    let completion = setup(2015, 7);

    let solution = solve("".to_string());

    completion(solution)
}
