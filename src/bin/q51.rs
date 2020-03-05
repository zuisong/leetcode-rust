/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N皇后
 *
 * https://leetcode-cn.com/problems/n-queens/description/
 *
 * algorithms
 * Hard (58.96%)
 * Total Accepted:    4.8K
 * Total Submissions: 8.1K
 * Testcase Example:  '4'
 *
 * n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 *
 *
 *
 * 上图为 8 皇后问题的一种解法。
 *
 * 给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。
 *
 * 每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
 *
 * 示例:
 *
 * 输入: 4
 * 输出: [
 * ⁠[".Q..",  // 解法 1
 * ⁠ "...Q",
 * ⁠ "Q...",
 * ⁠ "..Q."],
 *
 * ⁠["..Q.",  // 解法 2
 * ⁠ "Q...",
 * ⁠ "...Q",
 * ⁠ ".Q.."]
 * ]
 * 解释: 4 皇后问题存在两个不同的解法。
 *
 *
 */

extern crate log;

use leetcode_rust::init_logger;
use log::*;

struct Solution {}

const EMPTY_GRID: &str = "+ ";
const QUEEN_GRID: &str = "Q ";

impl Solution {
    pub fn dfs(v: &mut Vec<i32>, layer: i32, result: &mut Vec<Vec<String>>) {
        //        if result.len() > 0 {  // 找到一个就结束
        //            return;
        //        }
        //        info!("{:?}", v);

        let n = v.len() as i32;
        if layer == n {
            let res: Vec<String> = v
                .iter()
                .map(|it| {
                    let mut s = vec![EMPTY_GRID; n as usize];
                    s[*it as usize] = QUEEN_GRID;

                    let s = s.join("");
                    //                    info!("{}", s);
                    s
                })
                .collect();

            result.push(res);

            return;
        }

        for num in 0..n {
            let mut b = false;
            for i in 0..layer {
                if v[i as usize] == num || (v[i as usize] - num).abs() == (layer - i) {
                    b = true;
                    break;
                }
            }
            if !b {
                v[layer as usize] = num;
                Solution::dfs(v, layer + 1, result);
                v[layer as usize] = -1;
            }
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut v = vec![-1; n as usize];
        let mut res: Vec<Vec<String>> = vec![];
        Solution::dfs(&mut v, 0, &mut res);
        return res;
    }
}

fn main() {
    init_logger();

    let result = Solution::solve_n_queens(8);

    info!("{}", result.len());

    for (idx, res) in result.iter().enumerate() {
        info!("--> {}", idx + 1);
        for r in res {
            info!("{}", r);
        }
    }
}
