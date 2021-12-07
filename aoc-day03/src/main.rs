// https://adventofcode.com/2021/day/3

fn parse_lines(text: &str) -> (Vec<i32>, u8) {
    let nums = text
        .lines()
        .map(|line| i32::from_str_radix(line, 2).unwrap())
        .collect();
    (nums, text.lines().next().unwrap().len() as u8)
}

fn gamma_epsilon(nums: &[i32], power: u8) -> (i32, i32) {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in (0..power).rev() {
        let check = (1 << i) as i32;
        let (ones, zeroes): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&num| num & check != 0);
        if ones.len() > zeroes.len() {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    (gamma, epsilon)
}

use std::cmp::Ordering;

fn oxygen_rating(nums: &[i32], power: u8) -> i32 {
    let mut numbers = nums.to_vec();

    for i in (0..power).rev() {
        let check = (1 << i) as i32;
        let num_iter = numbers.iter();
        if num_iter.len() <= 1 {
            break;
        }
        let (ones, zeroes): (Vec<i32>, Vec<i32>) = num_iter.partition(|&num| num & check != 0);

        match ones.len().cmp(&zeroes.len()) {
            Ordering::Greater | Ordering::Equal => numbers = ones,
            Ordering::Less => numbers = zeroes,
        }
    }

    *numbers.last().unwrap()
}

fn co2_rating(nums: &[i32], power: u8) -> i32 {
    let mut numbers = nums.to_vec();

    for i in (0..power).rev() {
        let check = (1 << i) as i32;
        let num_iter = numbers.iter();
        if num_iter.len() <= 1 {
            break;
        }
        let (ones, zeroes): (Vec<i32>, Vec<i32>) = num_iter.partition(|&num| num & check != 0);

        match ones.len().cmp(&zeroes.len()) {
            Ordering::Greater | Ordering::Equal => numbers = zeroes,
            Ordering::Less => numbers = ones,
        }
    }

    *numbers.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gamma_epsilon_valid() {
        let (lines, power) = parse_lines(include_str!("../test_input.txt"));
        let expected = (22, 9);
        assert_eq!(expected, gamma_epsilon(&lines, power));
    }

    #[test]
    fn oxygen_rating_valid() {
        let (lines, power) = parse_lines(include_str!("../test_input.txt"));
        let expected = 23;
        assert_eq!(expected, oxygen_rating(&lines, power));
    }

    #[test]
    fn co2_rating_valid() {
        let (lines, power) = parse_lines(include_str!("../test_input.txt"));
        let expected = 10;
        assert_eq!(expected, co2_rating(&lines, power));
    }
}

fn main() {
    let (lines, power) = parse_lines(include_str!("../input.txt"));
    let (gamma, epsilon) = gamma_epsilon(&lines, power);
    println!("Part 1 Result: {}", gamma * epsilon);

    let oxygen = oxygen_rating(&lines, power);
    let co2 = co2_rating(&lines, power);
    println!("Part 2 Result: {}", oxygen * co2);
}
