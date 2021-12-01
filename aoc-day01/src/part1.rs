pub fn count_increased(measurements: &[usize]) -> usize {
    let mut increases = 0;
    if measurements.len() < 2 {
        return increases;
    }

    let mut measure_iter = measurements.iter();
    let mut prev = measure_iter.next().unwrap();

    for num in measure_iter {
        if num > prev {
            increases += 1;
        }
        prev = num;
    }

    increases
}

#[cfg(test)]
mod test {
    use super::count_increased;
    use crate::parse_lines;

    #[test]
    fn count_increased_valid() {
        let nums = parse_lines(include_str!("../test_input.txt"));
        let result = count_increased(&nums);
        assert_eq!(result, 7);
    }
}
