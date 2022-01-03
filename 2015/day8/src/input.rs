use helpers::read_lines;

pub fn parse(path: &str) -> Vec<String> {
    let lines = read_lines(path).unwrap();

    lines
        .map(|line| { line.unwrap() })
        .collect()
}
