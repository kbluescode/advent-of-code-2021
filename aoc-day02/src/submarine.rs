use super::Instruction;

pub struct Submarine {
    horizontal: u32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn apply_instruction_part_1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(num) => self.horizontal += num,
            Instruction::Down(num) => self.depth += *num as i32,
            Instruction::Up(num) => self.depth -= *num as i32,
        }
    }

    pub fn apply_instruction_part_2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(num) => {
                self.horizontal += num;
                self.depth += self.aim * *num as i32;
            }
            Instruction::Down(num) => self.aim += *num as i32,
            Instruction::Up(num) => self.aim -= *num as i32,
        }
    }

    pub fn combined_sum(&self) -> i32 {
        self.horizontal as i32 * self.depth
    }
}

#[cfg(test)]
mod test {
    use super::Submarine;

    #[test]
    fn combined_sum_valid() {
        let mut sub = Submarine::new();
        sub.horizontal = 5;
        sub.depth = 3;
        assert_eq!(sub.combined_sum(), 15);
    }

    mod part1 {
        use crate::instruction::Instruction;
        use crate::submarine::Submarine;

        #[test]
        fn apply_instruction_valid() {
            let mut sub = Submarine::new();
            let instruction = Instruction::Forward(5);
            sub.apply_instruction_part_1(&instruction);
            assert_eq!(sub.horizontal, 5);
            assert_eq!(sub.depth, 0);

            let instruction = Instruction::Down(3);
            sub.apply_instruction_part_1(&instruction);
            assert_eq!(sub.horizontal, 5);
            assert_eq!(sub.depth, 3);

            let instruction = Instruction::Up(8);
            sub.apply_instruction_part_1(&instruction);
            assert_eq!(sub.horizontal, 5);
            assert_eq!(sub.depth, -5);
        }
    }

    mod part2 {
        use crate::instruction::Instruction;
        use crate::submarine::Submarine;

        #[test]
        fn apply_instruction_valid() {
            let mut sub = Submarine::new();
            let instruction = Instruction::Forward(5);
            sub.apply_instruction_part_2(&instruction);
            assert_eq!(sub.horizontal, 5);
            assert_eq!(sub.depth, 0);
            assert_eq!(sub.aim, 0);

            let instruction = Instruction::Down(3);
            sub.apply_instruction_part_2(&instruction);
            assert_eq!(sub.horizontal, 5);
            assert_eq!(sub.depth, 0);
            assert_eq!(sub.aim, 3);

            let instruction = Instruction::Forward(2);
            sub.apply_instruction_part_2(&instruction);
            assert_eq!(sub.horizontal, 7);
            assert_eq!(sub.depth, 6);
            assert_eq!(sub.aim, 3);

            let instruction = Instruction::Up(8);
            sub.apply_instruction_part_2(&instruction);
            assert_eq!(sub.horizontal, 7);
            assert_eq!(sub.depth, 6);
            assert_eq!(sub.aim, -5);

            let instruction = Instruction::Forward(2);
            sub.apply_instruction_part_2(&instruction);
            assert_eq!(sub.horizontal, 9);
            assert_eq!(sub.depth, -4);
            assert_eq!(sub.aim, -5);
        }
    }
}
