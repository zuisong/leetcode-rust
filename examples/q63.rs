/**
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 *
 * https://leetcode-cn.com/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (30.86%)
 * Total Accepted:    8.6K
 * Total Submissions: 27.7K
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
 *
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
 *
 * 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
 *
 *
 *
 * 网格中的障碍物和空位置分别用 1 和 0 来表示。
 *
 * 说明：m 和 n 的值均不超过 100。
 *
 * 示例 1:
 *
 * 输入:
 * [
 * [0,0,0],
 * [0,1,0],
 * [0,0,0]
 * ]
 * 输出: 2
 * 解释:
 * 3x3 网格的正中间有一个障碍物。
 * 从左上角到右下角一共有 2 条不同的路径：
 * 1. 向右 -> 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右 -> 向右
 *
 *
 */
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let lenx = obstacle_grid.len();
        let leny = obstacle_grid.first().unwrap().len();
        let mut v = vec![0; leny];
        v[0] = 1;
        for i in 0..lenx {
            if obstacle_grid[i][0] == 1 {
                v[0] = 0;
            }
            for j in 1..leny {
                if obstacle_grid[i][j] == 1 {
                    v[j] = 0;
                } else {
                    v[j] += v[j - 1]
                }
            }
        }
        v[leny - 1]
    }
}

fn main() {
    let v = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
    let s = Solution::unique_paths_with_obstacles(v);
    dbg!(s);
}

struct Solution {}
