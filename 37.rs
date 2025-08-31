struct SudokuMap {
    row_map: Vec<u16>,
    col_map: Vec<u16>,
    matrix_map: Vec<u16>,
}

impl SudokuMap {
    fn new() -> Self {
        SudokuMap {
            row_map: vec![0; 9],
            col_map: vec![0; 9],
            matrix_map: vec![0; 9],
        }
    }
}

const zero_point : u8 = '0' as u8;

impl Solution {
    pub fn dfs(curr_board: &mut Vec<Vec<char>>, sudoku_map: &mut SudokuMap, nxt_i: usize, nxt_j: usize, solu: &mut bool) {
        if nxt_i == 9 {
            *solu = true;
            return;
        }

        if curr_board[nxt_i][nxt_j] != '.' {
            Self::dfs(curr_board, sudoku_map, nxt_i + (nxt_j + 1) / 9, (nxt_j + 1) % 9, solu);
            return;
        }

        // println!("{}, {}", nxt_i, nxt_j);

        for digital in 1..10 {
            if (sudoku_map.row_map[nxt_i] & (1 << digital)) != 0 {
                continue;
            }
            if (sudoku_map.col_map[nxt_j]  & (1 << digital)) != 0 {
                continue;
            }
            let matrix_idx = (nxt_i / 3) * 3 + nxt_j /3;
            if (sudoku_map.matrix_map[matrix_idx] & (1 << digital)) != 0 {
                continue;
            }
            // println!("i={},j={}, digital={}, row_map={}, col_map={}, matrix_map={}", nxt_i, nxt_j, digital, sudoku_map.row_map[nxt_i], sudoku_map.col_map[nxt_j], sudoku_map.matrix_map[(nxt_i / 3) * 3 + nxt_j /3]);
            sudoku_map.row_map[nxt_i] ^= 1 << digital;
            sudoku_map.col_map[nxt_j] ^= 1 << digital;
            sudoku_map.matrix_map[matrix_idx] ^= 1 << digital;
            curr_board[nxt_i][nxt_j] = (digital + zero_point) as char;
            // println!("curr_board[i][j]={}", curr_board[nxt_i][nxt_j]);
            Self::dfs(curr_board, sudoku_map, nxt_i + (nxt_j + 1) / 9, (nxt_j + 1) % 9, solu);
            if *solu == true {
                break;
            }
            sudoku_map.row_map[nxt_i] ^= 1 << digital;
            sudoku_map.col_map[nxt_j] ^= 1 << digital;
            sudoku_map.matrix_map[matrix_idx] ^= 1 << digital;
            curr_board[nxt_i][nxt_j] = '.';
        }
    }

    pub fn get_bin(digital: char) -> u16 {
        return (1 << ((digital as u8) - zero_point)) as u16;
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku_map = SudokuMap::new();

        let mut nxt_i: usize = 0;
        let mut nxt_j: usize = 0;
        
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let mut bin_num = Self::get_bin(board[i][j]);
                    // println!("bin_num={}, i={}, j={}, matrix_idx={}", bin_num, i, j, (i / 3) * 3 + j / 3);
                    sudoku_map.row_map[i] ^= bin_num;
                    sudoku_map.col_map[j] ^= bin_num;
                    sudoku_map.matrix_map[(i / 3) * 3 + j / 3] ^= bin_num;
                    // println!("bin_num={}, i={}, j={}, matrix_idx={}, row_map={}, col_map={}, matrix_map={}", bin_num, i, j, (i / 3) * 3 + j / 3, sudoku_map.row_map[i], sudoku_map.row_map[j], sudoku_map.row_map[(i / 3) * 3 + j /3]);
                }
            }
        }

        let mut solu = false;
        Self::dfs(board, &mut sudoku_map, nxt_i, nxt_j, &mut solu);
    }
}
