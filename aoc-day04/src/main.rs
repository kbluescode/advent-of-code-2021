// https://adventofcode.com/2021/day/4

const GRID_SIZE: usize = 5;

mod board;
pub use board::Board;

fn make_boards(lines: &str) -> (Vec<u32>, Vec<Board>) {
    let mut line_iter = lines.lines();
    let numbers = line_iter.next().unwrap();
    line_iter.next();

    let numbers: Vec<u32> = numbers
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut board_lines = Vec::new();
    for line in line_iter {
        if line.is_empty() {
            continue;
        }
        board_lines.push(line);
        if board_lines.len() == GRID_SIZE {
            let board = Board::from_lines(&board_lines);
            boards.push(board);
            board_lines.clear();
        }
    }

    (numbers, boards)
}

fn part_one(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut winner_info: Option<(u32, Board)> = None;
    let mut boards: Vec<Board> = boards.iter().copied().collect();

    for num in numbers {
        boards.iter_mut().for_each(|board| board.set_marked(*num));

        if let Some(&board) = boards.iter().find(|&board| board.has_won()) {
            winner_info = Some((*num, board));
            break;
        }
    }

    let (num, board) = winner_info.unwrap();
    let sum: u32 = board.unmarked().iter().map(|cell| cell.value).sum();
    num * sum
}

fn part_two(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut winners: Vec<(u32, Board)> = vec![];
    let mut remaining: Vec<Board> = boards.iter().copied().collect();

    for num in numbers {
        remaining
            .iter_mut()
            .for_each(|board| board.set_marked(*num));
        let (new_winners, new_remaining): (Vec<Board>, Vec<Board>) =
            remaining.iter().partition(|board| board.has_won());
        remaining = new_remaining;

        winners.extend(new_winners.iter().map(|&board| (*num, board)));
    }
    let (num, last_winner) = winners.last().unwrap();
    let sum: u32 = last_winner.unmarked().iter().map(|&cell| cell.value).sum();
    num * sum
}

fn main() {
    let (numbers, boards) = make_boards(include_str!("../input.txt"));
    println!("Part 1: {}", part_one(&numbers, &boards));
    println!("Part 2: {}", part_two(&numbers, &boards));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let (numbers, boards) = make_boards(include_str!("../test_input.txt"));
        let expected = 4512;
        assert_eq!(super::part_one(&numbers, &boards), expected);
    }

    #[test]
    fn part_two() {
        let (numbers, boards) = make_boards(include_str!("../test_input.txt"));
        let expected = 1924;
        assert_eq!(super::part_two(&numbers, &boards), expected);
    }
}
