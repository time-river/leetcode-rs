// https://leetcode.com/problems/valid-sudoku/

use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..board.len() {
            let mut row: HashSet<char> = HashSet::new();
            let mut col: HashSet<char> = HashSet::new();
            for j in 0..board[0].len() {
                if board[i][j] != '.' {
                    match row.get(&board[i][j]) {
                        None => {
                            row.insert(board[i][j]);
                        }
                        _ => {
                            return false;
                        }
                    }
                }
                if board[j][i] != '.' {
                    match col.get(&board[j][i]) {
                        None => {
                            col.insert(board[j][i]);
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
        }

        for i in (0..board.len()).step_by(3) {
            for j in (0..board[0].len()).step_by(3) {
                let mut bucket: HashSet<char> = HashSet::new();

                for x in i..(i+3) {
                    for y in j..(j+3) {
                        if board[x][y] != '.' {
                            match bucket.get(&board[x][y]) {
                                None => {
                                    bucket.insert(board[x][y]);
                                }
                                _ => {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']];
        assert_eq!(Solution::is_valid_sudoku(board), false);

        let board: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']];
        assert_eq!(Solution::is_valid_sudoku(board), false);

    }
}
