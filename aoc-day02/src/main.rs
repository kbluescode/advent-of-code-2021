// https://adventofcode.com/2021/day/2

mod instruction;
use instruction::Instruction;

mod submarine;
use submarine::Submarine;

fn main() {
    let instructions = instruction_lines(include_str!("../input.txt"));
    let mut sub = Submarine::new();
    for instruction in &instructions {
        sub.apply_instruction_part_1(instruction);
    }
    println!("Part 1\nResult: {}", sub.combined_sum());

    let mut sub = Submarine::new();
    for instruction in &instructions {
        sub.apply_instruction_part_2(instruction);
    }
    println!("Part 2\nResult: {}", sub.combined_sum());
}

fn instruction_lines(text: &str) -> Vec<Instruction> {
    text.split('\n').map(Instruction::from_line).collect()
}

#[cfg(test)]
mod test {

    mod part1 {
        use crate::instruction_lines;
        use crate::submarine::Submarine;

        #[test]
        fn test_input() {
            let instructions = instruction_lines(include_str!("../test_input.txt"));
            let mut sub = Submarine::new();
            for instruction in instructions {
                sub.apply_instruction_part_1(&instruction);
            }
            assert_eq!(sub.combined_sum(), 150);
        }
    }

    mod part2 {
        use crate::instruction_lines;
        use crate::submarine::Submarine;

        #[test]
        fn test_input() {
            let instructions = instruction_lines(include_str!("../test_input.txt"));
            let mut sub = Submarine::new();
            for instruction in instructions {
                sub.apply_instruction_part_2(&instruction);
            }
            assert_eq!(sub.combined_sum(), 900);
        }
    }
}
