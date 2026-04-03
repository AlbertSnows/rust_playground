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

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // let vertical_set = board
    //     .iter()
    //     .enumerate()
    //     .fold(
    //         Vec::new(),
    //         |mut ?, (vertical_index, horizontal_row)| {
    //             // (0, [5, 3, .])
    //             let vertical_row = horizontal_row.iter().enumerate().fold(
    //                 Vec::new(),
    //                 |mut vertical_row, (horizontal_index, value)| {
    //                     // (1, 3)
    //                     //
    //                     vertical_row.push(board[horizontal_index][vertical_index]);
    //                     vertical_row
    //                 },
    //             ); // [5,6,.]
    //             ?.push(new_row);
    //             ?
    //         },
    //     );
    Default::default()
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
