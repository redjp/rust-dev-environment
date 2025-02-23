use std::io::BufRead;

pub fn solve(input: &mut dyn BufRead) -> Vec<String> {
    input.lines().map(|line| line.unwrap().to_uppercase()).collect()
}