// https://adventofcode.com/2021/day/1

mod part1;
mod part2;

fn main() {
    let nums = parse_lines(include_str!("../input.txt"));
    let result = part1::count_increased(&nums);
    println!("Part 1\nResult: {}", result);

    let result = part2::count_window_increased(&nums);
    println!("Part 2\nResult: {}", result);
}

fn parse_lines(lines: &str) -> Vec<usize> {
    lines
        .split('\n')
        .map(|line| match line.parse::<usize>() {
            Ok(num) => num,
            Err(err) => panic!("Error: {}", err),
        })
        .collect()
}
