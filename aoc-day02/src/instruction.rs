#[derive(Debug, PartialEq)]
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        let mut chunks = line.split_whitespace().take(2);
        let dir = chunks.next().unwrap();
        let num = chunks.next().unwrap();
        let num = num.parse::<u32>().expect("Couldn't convert number");
        match dir {
            "forward" => Instruction::Forward(num),
            "down" => Instruction::Down(num),
            "up" => Instruction::Up(num),
            _ => panic!("No match for {}", dir),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn from_line_invalid_text() {
        Instruction::from_line("blah 22");
    }

    #[test]
    #[should_panic]
    fn from_line_invalid_number() {
        Instruction::from_line("forward az");
    }

    #[test]
    fn from_line_valid() {
        let expected = vec![
            Instruction::Forward(5),
            Instruction::Down(3),
            Instruction::Up(50),
        ];

        let input = vec!["forward 5", "down 3", "up 50"];

        for i in 0..expected.len() {
            assert_eq!(Instruction::from_line(input[i]), expected[i]);
        }
    }
}
