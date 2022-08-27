const SOLUTIONS: [u16; 8] = [
    0b111_000_000,
    0b000_111_000,
    0b000_000_111,
    0b100_100_100,
    0b010_010_010,
    0b001_001_001,
    0b100_010_001,
    0b001_010_100,
];

pub fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    fn to_binary(board: &[&[u8; 3]; 3], player: u8) -> (u16, bool) {
        let mut binary = 0_u16;
        let mut has_zero = false;

        for r in 0..3 {
            for c in 0..3 {
                if board[r][c] == 0 {
                    has_zero = true;
                }
                if board[r][c] == player {
                    binary = binary | 1_u16 << (3 * r + c);
                }
            }
        }

        (binary, has_zero)
    }

    let (board_x, contains_zero) = to_binary(board, 1);
    let (board_o, _) = to_binary(board, 2);

    for solution in SOLUTIONS {
        if board_x & solution == solution {
            return 1;
        }
        if board_o & solution == solution {
            return 2;
        }
    }

    return if contains_zero { -1 } else { 0 };
}

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(
            actual == expected,
            "With board = {board:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
        ] {
            dotest(&board, expected);
        }
    }
}
