const WINDOW_SIZE: usize = 3;

use super::part1::count_increased;

pub fn count_window_increased(measurements: &[usize]) -> usize {
    if measurements.len() < WINDOW_SIZE * 2 {
        return 0;
    }

    let mut sums = Vec::new();
    let max_idx = measurements.len() - WINDOW_SIZE;

    for i in 0..=max_idx {
        sums.push(measurements[i..i + WINDOW_SIZE].iter().sum());
    }

    count_increased(&sums)
}

#[cfg(test)]
mod test {
    use super::count_window_increased;
    use crate::parse_lines;

    #[test]
    fn count_window_increased_valid() {
        let nums = parse_lines(include_str!("../test_input.txt"));
        let result = count_window_increased(&nums);
        assert_eq!(result, 5);
    }
}
