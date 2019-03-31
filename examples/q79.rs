/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 *
 * https://leetcode-cn.com/problems/word-search/description/
 *
 * algorithms
 * Medium (36.08%)
 * Total Accepted:    7.8K
 * Total Submissions: 21.4K
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * 给定一个二维网格和一个单词，找出该单词是否存在于网格中。
 *
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
 *
 * 示例:
 *
 * board =
 * [
 * ⁠ ['A','B','C','E'],
 * ⁠ ['S','F','C','S'],
 * ⁠ ['A','D','E','E']
 * ]
 *
 * 给定 word = "ABCCED", 返回 true.
 * 给定 word = "SEE", 返回 true.
 * 给定 word = "ABCB", 返回 false.
 *
 */
use std::collections::HashSet;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &Vec<Vec<char>>,
            word: &Vec<char>,
            pos: (i32, i32),
            route: &mut HashSet<(i32, i32)>,
            idx: usize,
        ) -> bool {
            let lx = board.len() as i32;
            let ly = board.first().unwrap().len() as i32;
            if idx == word.len() {
                return true;
            }
            if pos.0 < 0 || pos.0 >= lx {
                return false;
            }
            if pos.1 < 0 || pos.1 >= ly {
                return false;
            }
            if board[pos.0 as usize][pos.1 as usize] != word[idx] {
                return false;
            }
            route.insert(pos);
            for (i, j) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let i = *i;
                let j = *j;
                let new_pos = (pos.0 + i, pos.1 + j);
                if route.contains(&new_pos) {
                    continue;
                };
                let b = dfs(board, word, new_pos, route, idx + 1);
                if b {
                    return true;
                };
            }
            route.remove(&pos);
            false
        }
        let word: Vec<_> = word.chars().collect();
        let lx = board.len();
        let ly = board.first().unwrap().len();
        for i in 0..lx {
            for j in 0..ly {
                let b = dfs(&board, &word, (i as i32, j as i32), &mut HashSet::new(), 0);
                if b {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    fn check(b: bool, s: &str) {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(b, Solution::exist(board, s.to_string()));
    }
    check(true, "ABCCED");
    check(true, "SEE");
    check(false, "ABCB");
    //        * 给定 word = "ABCCED", 返回 true.
    //        * 给定 word = "SEE", 返回 true.
    //        * 给定 word = "ABCB", 返回 false.
}

struct Solution {}
