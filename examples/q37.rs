extern crate log;
extern crate simple_logger;

use log::*;

struct Solution {}

impl Solution {
    #[inline]
    fn get_row_col(n: usize) -> (usize, usize) {
        let row = n / 9;
        let col = n % 9;
        return (row, col);
    }

    fn check_num(c: char, n: usize, board: &mut Vec<Vec<char>>) -> bool {
        Self::check_row_col(c, n, board) && Self::check_grid(c, n, board)
    }

    fn check_row_col(c: char, n: usize, board: &mut Vec<Vec<char>>) -> bool {
        let (row, col) = Self::get_row_col(n);

        for i in 0..9 {
            if board[row][i] == c || board[i][col] == c {
                return false;
            }
        }
        return true;
    }
    fn check_grid(c: char, n: usize, board: &mut Vec<Vec<char>>) -> bool {
        let grid_row = n / 27;
        let grid_col = (n % 9) / 3;
        for i in (grid_row * 3)..(grid_row * 3 + 3) {
            for j in (grid_col * 3)..(grid_col * 3 + 3) {
                if board[i][j] == c {
                    return false;
                }
            }
        }
        return true;
    }

    fn dfs(n: usize, board: &mut Vec<Vec<char>>) -> bool {
        if n == 81 {
            return true;
        }

        let (row, col) = Self::get_row_col(n);

        if board[row][col] != '.' {
            return Self::dfs(n + 1, board);
        }

        for c in ['1', '2', '3', '4', '5', '6', '7', '8', '9'].iter() {
            if Self::check_num(*c, n, board) {
                board[row][col] = *c;
                let res = Self::dfs(n, board);
                if !res {
                    board[row][col] = '.';
                } else {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::dfs(0, board);
    }
}

fn main() {
    simple_logger::init().unwrap();

    let mut board: Vec<Vec<char>> = vec![
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
    board.iter().for_each(|line| info!("{:?}", line));
    info!("===");
    Solution::solve_sudoku(&mut board);
    board.iter().for_each(|line| info!("{:?}", line));
}
