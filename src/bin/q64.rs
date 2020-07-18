/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 *
 * https://leetcode-cn.com/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (58.95%)
 * Total Accepted:    11.5K
 * Total Submissions: 19.4K
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * 给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
 *
 * 说明：每次只能向下或者向右移动一步。
 *
 * 示例:
 *
 * 输入:
 * [
 * [1,3,1],
 * ⁠ [1,5,1],
 * ⁠ [4,2,1]
 * ]
 * 输出: 7
 * 解释: 因为路径 1→3→1→1→1 的总和最小。
 *
 *
 */
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let lenx = grid.len();
        let leny = grid.first().unwrap().len();
        let mut dp = vec![vec![0; leny]; lenx];
        dp[0][0] = grid[0][0];

        for i in 1..lenx {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for i in 1..leny {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        for i in 1..lenx {
            for j in 1..leny {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j]
            }
        }
        dp[lenx - 1][leny - 1]
    }
}

fn main() {
    let v = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

    let s = Solution::min_path_sum(v);
    dbg!(s);
}

struct Solution {}
