struct Sudoku<'a> {
    field: &'a mut [[u8; 9]; 9],
}

impl<'a> Sudoku<'a> {
    pub fn new(field: &'a mut [[u8; 9]; 9]) -> Self {
        Sudoku { field }
    }

    fn solve(&mut self) -> bool {
        let (row, col) = self.next_cell();

        for n in 1..=9 {
            if self.can_set(n, row, col) {
                self.field[row][col] = n;

                if !self.solve() {
                    self.field[row][col] = 0;
                }
            }
        }
        self.field[row][col] != 0
    }

    fn can_set(&self, n: u8, row: usize, col: usize) -> bool {
        !self.in_row(n, row) && !self.in_col(n, col) && !self.in_block(n, row, col)
    }

    fn in_row(&self, n: u8, row: usize) -> bool {
        for col in 0..9 {
            if self.field[row][col] == n {
                return true;
            }
        }
        false
    }

    fn in_col(&self, n: u8, col: usize) -> bool {
        for row in 0..9 {
            if self.field[row][col] == n {
                return true;
            }
        }
        false
    }

    fn in_block(&self, n: u8, row: usize, col: usize) -> bool {
        let block_row = row / 3 * 3;
        let block_col = col / 3 * 3;
        for row in block_row..block_row + 3 {
            for col in block_col..block_col + 3 {
                if self.field[row][col] == n {
                    return true;
                }
            }
        }
        false
    }

    fn next_cell(&self) -> (usize, usize) {
        // Find the block with the least number of 0's.
        // However, in the examples, they all have the\
        // same number of zeroes (5).
        let mut min_zeroes = 9;
        let mut min_zero_block = 0;

        for block in 0..9 {
            let (block_row, block_col) = Sudoku::block_origin(block);

            let mut zeroes_in_block = 0;

            for row in block_row..block_row + 3 {
                for col in block_col..block_col + 3 {
                    if self.field[row][col] == 0 {
                        zeroes_in_block += 1;
                    }
                }
            }

            if zeroes_in_block > 0 && zeroes_in_block < min_zeroes {
                min_zero_block = block;
                min_zeroes = zeroes_in_block;
            }
        }

        // Find the cell within the block that has the least number
        // of 0's either in the same row or in the same column.
        let (block_row, block_col) = Sudoku::block_origin(min_zero_block);

        let mut min_zeroes_row = usize::MAX;
        let mut min_zeroes_col = usize::MAX;
        let mut min_row_cell = (block_row, block_col);
        let mut min_col_cell = (block_row, block_col);

        for row in block_row..block_row + 3 {
            for col in block_col..block_col + 3 {
                if self.field[row][col] == 0 {
                    let zeroes_in_row = self.zeroes_in_row(row);
                    let zeroes_in_col = self.zeroes_in_col(col);

                    if zeroes_in_row < min_zeroes_row {
                        min_row_cell = (row, col);
                        min_zeroes_row = zeroes_in_row;
                    }

                    if zeroes_in_col < min_zeroes_col {
                        min_col_cell = (row, col);
                        min_zeroes_col = zeroes_in_col;
                    }
                }
            }
        }

        if min_zeroes_row < min_zeroes_col {
            min_row_cell
        } else {
            min_col_cell
        }
    }

    fn zeroes_in_row(&self, row: usize) -> usize {
        self.field[row].iter().filter(|&c| *c == 0).count()
    }

    fn zeroes_in_col(&self, col: usize) -> usize {
        self.field.iter().map(|row| row[col] == 0).count()
    }

    fn block_origin(block: usize) -> (usize, usize) {
        (block / 3 * 3, block % 3 * 3)
    }
}

pub fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let mut s = Sudoku::new(puzzle);
    s.solve();
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn test_helper_fns() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let sudoku = Sudoku::new(&mut puzzle);

        assert!(sudoku.in_row(7, 0));
        assert!(!sudoku.in_row(3, 2));

        assert!(sudoku.in_col(9, 1));
        assert!(!sudoku.in_col(1, 4));

        assert!(sudoku.in_block(2, 0, 0));
        assert!(sudoku.in_block(2, 2, 2));
        assert!(sudoku.in_block(9, 4, 4));
        assert!(!sudoku.in_block(1, 4, 4));
        assert!(!sudoku.in_block(3, 8, 8));
    }

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_3() {
        let mut puzzle = [
            [0, 1, 9, 0, 6, 0, 5, 4, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [8, 2, 0, 9, 7, 4, 0, 3, 6],
            [0, 0, 1, 5, 0, 3, 8, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 2, 7, 0, 1, 6, 0, 0],
            [7, 5, 0, 1, 3, 8, 0, 9, 2],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 3, 0, 4, 0, 7, 1, 0],
        ];

        let solution = [
            [3, 1, 9, 8, 6, 2, 5, 4, 7],
            [4, 6, 7, 3, 1, 5, 2, 8, 9],
            [8, 2, 5, 9, 7, 4, 1, 3, 6],
            [6, 7, 1, 5, 9, 3, 8, 2, 4],
            [5, 3, 8, 4, 2, 6, 9, 7, 1],
            [9, 4, 2, 7, 8, 1, 6, 5, 3],
            [7, 5, 6, 1, 3, 8, 4, 9, 2],
            [1, 9, 4, 2, 5, 7, 3, 6, 8],
            [2, 8, 3, 6, 4, 9, 7, 1, 5],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }
}
