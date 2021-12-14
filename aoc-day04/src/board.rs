use super::GRID_SIZE;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    marked: bool,
    pub value: u32,
}

impl Cell {
    fn new(value: u32) -> Self {
        Self {
            marked: false,
            value,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    marked_count: u32,
}

impl Board {
    pub fn from_lines(lines: &[&str]) -> Self {
        let mut grid: [[Cell; GRID_SIZE]; GRID_SIZE] = [[Cell::new(0); 5]; 5];

        let lines = lines.iter().enumerate();
        for (i, line) in lines {
            let nums: Vec<Cell> = line
                .split_whitespace()
                .map(|num| Cell::new(num.parse::<u32>().unwrap()))
                .collect();
            for j in 0..GRID_SIZE {
                let item = nums.get(j).unwrap();
                grid[i][j] = *item;
            }
        }

        Self {
            grid,
            marked_count: 0,
        }
    }

    pub fn set_marked(&mut self, num: u32) {
        for row in &mut self.grid {
            for mut col in row {
                if col.value == num && !col.marked {
                    col.marked = true;
                    self.marked_count += 1;
                    return;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        if self.marked_count < GRID_SIZE as u32 {
            return false;
        }

        for col_idx in 0..GRID_SIZE {
            let count = (0..GRID_SIZE).fold(0, |acc, row_idx| {
                let cell = self.grid[row_idx][col_idx];
                if cell.marked {
                    acc + 1
                } else {
                    acc
                }
            });
            if count == GRID_SIZE {
                return true;
            }
        }

        // check rows
        for row in self.grid {
            let count = row
                .iter()
                .fold(0, |acc, cell| if cell.marked { acc + 1 } else { acc });
            if count == GRID_SIZE {
                return true;
            }
        }

        false
    }

    pub fn marked(&self) -> Vec<&Cell> {
        let mut cells = Vec::new();

        self.grid.iter().for_each(|row| {
            row.iter()
                .filter(|&cell| cell.marked)
                .for_each(|cell| cells.push(cell));
        });

        cells
    }

    pub fn unmarked(&self) -> Vec<&Cell> {
        let mut cells = Vec::new();

        self.grid.iter().for_each(|row| {
            row.iter()
                .filter(|&cell| !cell.marked)
                .for_each(|cell| cells.push(cell));
        });

        cells
    }
}

#[cfg(test)]
mod test {
    const GOOD_BOARD: &str = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
";

    use super::*;

    #[test]
    fn from_lines_valid() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let board = Board::from_lines(&lines);
        let expected = [
            [
                Cell::new(22),
                Cell::new(13),
                Cell::new(17),
                Cell::new(11),
                Cell::new(0),
            ],
            [
                Cell::new(8),
                Cell::new(2),
                Cell::new(23),
                Cell::new(4),
                Cell::new(24),
            ],
            [
                Cell::new(21),
                Cell::new(9),
                Cell::new(14),
                Cell::new(16),
                Cell::new(7),
            ],
            [
                Cell::new(6),
                Cell::new(10),
                Cell::new(3),
                Cell::new(18),
                Cell::new(5),
            ],
            [
                Cell::new(1),
                Cell::new(12),
                Cell::new(20),
                Cell::new(15),
                Cell::new(19),
            ],
        ];
        assert_eq!(expected, board.grid);
    }

    #[test]
    fn set_marked_valid() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let mut board = Board::from_lines(&lines);
        assert_eq!(board.marked_count, 0);

        board.set_marked(2);
        assert_eq!(board.marked_count, 1);
    }

    #[test]
    fn has_won_rows() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let mut board = Board::from_lines(&lines);
        assert!(!board.has_won());

        for num in [8, 2, 23, 4, 24] {
            board.set_marked(num);
        }
        assert_eq!(board.marked_count, 5);
        assert!(board.has_won());
    }

    #[test]
    fn has_won_columns() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let mut board = Board::from_lines(&lines);
        assert!(!board.has_won());

        for num in [22, 8, 21, 6, 1] {
            board.set_marked(num);
        }
        assert_eq!(board.marked_count, 5);
        assert!(board.has_won());
    }

    #[test]
    fn marked_valid() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let mut board = Board::from_lines(&lines);

        assert_eq!(board.marked(), Vec::<&Cell>::new());

        let nums = vec![8, 2, 23, 4, 24];
        nums.iter().for_each(|&num| board.set_marked(num));
        assert_eq!(board.marked_count, 5);
        let results: Vec<u32> = board.marked().iter().map(|&cell| cell.value).collect();
        assert_eq!(results, nums);
    }

    #[test]
    fn unmarked_valid() {
        let lines: Vec<&str> = GOOD_BOARD.lines().collect();
        let mut board = Board::from_lines(&lines);

        assert_eq!(board.marked(), Vec::<&Cell>::new());

        let nums = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5,
        ];
        nums.iter().for_each(|&num| board.set_marked(num));
        let expected = vec![1, 12, 20, 15, 19];
        let results: Vec<u32> = board.unmarked().iter().map(|&cell| cell.value).collect();
        assert_eq!(results, expected);
    }
}
