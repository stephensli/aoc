use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution {
    pub part1: String,
    pub part2: String,
}

/// Helper to read lines into a iterator
///
/// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
///
/// # Arguments
///
/// * `filename`: The source file path.
///
/// returns: Result<Lines<BufReader<File>>, Error>
///
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Setup configures the day for running and allowing the solution to be
/// printed as expected. This is a standard way of outputting content.
///
/// It will also change the working directory to the source folder.
///
/// # Arguments
///
/// * `year`: The year in which the aoc was created.
/// * `day`: The day that is being exucted.
///
/// returns: Box<dyn Fn(Solution), Global>
///
/// # Examples
/// ```
/// use helpers::Solution;
/// let completion = setup(2015, 1);
///
/// // code to complete day
///
/// completion(Solution{ part1: "part1".to_string(), part2: "part2".to_string()})
/// ```
pub fn setup(year: i32, day: i32) -> Box<dyn Fn(Solution)> {
    let now = Instant::now();

    let current_directory = env::current_dir().unwrap();

    let target_path = format!("src");
    let target_directory = Path::new(&target_path);

    let project_directory = current_directory.join(target_directory);

    env::set_current_dir(&project_directory).unwrap();

    Box::new(move |solution: Solution| {
        println!("part1: {}", solution.part1);
        println!("part2: {}\n", solution.part2);

        println!("AOC ({}:{}): {:2?}", year, day, now.elapsed())
    })
}
