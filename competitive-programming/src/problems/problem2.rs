use std::io::BufRead;

pub fn solve(input: &mut dyn BufRead) -> Vec<String> {
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    (0..t)
        .map(|_| {
            let numbers: Vec<i32> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (numbers[0] + numbers[1]).to_string()
        })
        .collect()
}