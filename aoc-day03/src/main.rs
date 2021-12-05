// https://adventofcode.com/2021/day/3

const ONE: char = '1';
const ZERO: char = '0';

fn parse_lines(text: &str) -> Vec<&str> {
    text.split('\n').collect()
}

fn gamma_epsilon(lines: &[&str]) -> (i32, i32) {
    let mut stack = Vec::new();
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..lines[0].len() {
        for line in lines {
            stack.push(line.chars().nth(i).unwrap());
        }
        let (ones, zeroes): (Vec<char>, Vec<char>) = stack.iter().partition(|num| **num == ONE);
        if ones.len() > zeroes.len() {
            gamma.push(ONE);
            epsilon.push(ZERO);
        } else {
            gamma.push(ZERO);
            epsilon.push(ONE);
        }
        stack.clear();
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    (gamma, epsilon)
}

use std::cmp::Ordering;

fn find_match(lines: &[&str], positive: char, negative: char) -> i32 {
    let mut matches: Vec<&str> = lines.iter().copied().collect();

    for idx in 0..lines[0].len() {
        let match_iter = matches.iter();
        if match_iter.len() == 1 {
            break;
        }
        let (ones, zeroes): (Vec<&str>, Vec<&str>) =
            match_iter.partition(|&&line| line.chars().nth(idx).unwrap() == ONE);

        let matcher = match ones.len().cmp(&zeroes.len()) {
            Ordering::Greater => positive,
            Ordering::Less => negative,
            Ordering::Equal => positive,
        };

        matches.retain(|&line| line.chars().nth(idx).unwrap() == matcher);
    }

    i32::from_str_radix(matches.last().unwrap(), 2).unwrap()
}

fn oxygen_rating(lines: &[&str]) -> i32 {
    find_match(lines, ONE, ZERO)
}

fn co2_rating(lines: &[&str]) -> i32 {
    find_match(lines, ZERO, ONE)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gamma_epsilon_valid() {
        let lines = parse_lines(include_str!("../test_input.txt"));
        let expected = (22, 9);
        assert_eq!(expected, gamma_epsilon(&lines));
    }

    #[test]
    fn oxygen_rating_valid() {
        let lines = parse_lines(include_str!("../test_input.txt"));
        let expected = 23;
        assert_eq!(expected, oxygen_rating(&lines));
    }

    #[test]
    fn co2_rating_valid() {
        let lines = parse_lines(include_str!("../test_input.txt"));
        let expected = 10;
        assert_eq!(expected, co2_rating(&lines));
    }
}

fn main() {
    let lines = parse_lines(include_str!("../input.txt"));
    let (gamma, epsilon) = gamma_epsilon(&lines);
    println!("Part 1 Result: {}", gamma * epsilon);

    let oxygen = oxygen_rating(&lines);
    let co2 = co2_rating(&lines);
    println!("Part 2 Result: {}", oxygen * co2);
}
