use crate::common::{
    collections::{get_at, transpose},
    maps::has_dupes,
};

// Determine if a 9x9 Sudoku board is valid based on the rules: each row, column, and 3x3 box contains digits 1-9 with no repetition.

// ## Example
// Input: board with digits and '.' for empty cells
// Output: true (if valid), false (if invalid)
//
// get all left<->right
// get all top<->bottom
// for b[0-n][h]
//
// get all 3x3 boxes
// [0][0], [0][3], [0][6]
// [3][0], [3][3], [3][6]
// [6][0], [6][3], [6][6]

// [["5","3",".",".","7",".",".",".","."]
// ,["6",".",".","1","9","5",".",".","."]
// ,[".","9","8",".",".",".",".","6","."]
// ,["8",".",".",".","6",".",".",".","3"]
// ,["4",".",".","8",".","3",".",".","1"]
// ,["7",".",".",".","2",".",".",".","6"]
// ,[".","6",".",".",".",".","2","8","."]
// ,[".",".",".","4","1","9",".",".","5"]
// ,[".",".",".",".","8",".",".","7","9"]]
const RELATIVE_LOCATIONS: [(usize, usize); 9] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (2, 0),
    (1, 1),
    (1, 2),
    (2, 1),
    (2, 2),
];

const BOX_STARTING_LOCATIONS: [(usize, usize); 9] = [
    (0, 0),
    (0, 3),
    (0, 6),
    (3, 0),
    (3, 3),
    (3, 6),
    (6, 0),
    (6, 3),
    (6, 6),
];

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let only_nums = |row: &Vec<char>| {
        row.iter()
            .filter(|c| c.is_ascii_digit())
            .copied()
            .collect::<Vec<char>>()
    };
    let is_valid_row = |row: &Vec<char>| !has_dupes(only_nums(row).into_iter());
    // verify horizontal = easy, look row by row
    let horizontal_valid = board.iter().all(is_valid_row);

    // verify vertical = transpose, look row by row
    let transposed = transpose(&board);
    let vertical_valid = transposed.iter().all(is_valid_row);

    // verify boxes = [(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (1, 1), (1, 2), (2, 1), (2, 2)]
    let define_box_from_starting_location = |&(row, col)| {
        RELATIVE_LOCATIONS
            .iter()
            .map(|(r, c)| (row + r, col + c))
            .collect::<Vec<(usize, usize)>>()
    };
    let box_indexes: Vec<Vec<(usize, usize)>> = BOX_STARTING_LOCATIONS
        .iter()
        .map(define_box_from_starting_location)
        .collect();
    let boxes_as_rows: Vec<Vec<char>> = box_indexes
        .iter()
        .map(|index_row| {
            index_row
                .iter()
                .map(|(row, col)| board[*row][*col])
                .collect()
        })
        .collect();
    let boxes_valid = boxes_as_rows.iter().all(is_valid_row);

    println!("{:?}", boxes_as_rows);
    horizontal_valid && vertical_valid && boxes_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_board() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }

    #[test]
    fn invalid_board() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }
}
