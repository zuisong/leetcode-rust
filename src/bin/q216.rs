/**
 * @lc app=leetcode.cn id=216 lang=rust
 *
 * [216] 组合总和 III
 *
 * https://leetcode-cn.com/problems/combination-sum-iii/description/
 *
 * algorithms
 * Medium (63.45%)
 * Total Accepted:    3.1K
 * Total Submissions: 4.9K
 * Testcase Example:  '3\n7'
 *
 * 找出所有相加之和为 n 的 k 个数的组合。组合中只允许含有 1 - 9 的正整数，并且每种组合中不存在重复的数字。
 *
 * 说明：
 *
 *
 * 所有数字都是正整数。
 * 解集不能包含重复的组合。
 *
 *
 * 示例 1:
 *
 * 输入: k = 3, n = 7
 * 输出: [[1,2,4]]
 *
 *
 * 示例 2:
 *
 * 输入: k = 3, n = 9
 * 输出: [[1,2,6], [1,3,5], [2,3,4]]
 *
 *
 */
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::dfs(&mut res, &mut vec![], n, k as usize);
        res
    }
    pub fn dfs(res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, target: i32, k: usize) {
        if temp.len() == k {
            if target == 0 {
                res.push(temp.clone());
            }
            return;
        }
        let start = temp.last().map(|it| it + 1).unwrap_or(1);
        for i in start..10 {
            if !temp.contains(&i) {
                temp.push(i);
                Self::dfs(res, temp, target - i, k);
                temp.pop();
            }
        }
    }
}

fn main() {
    let res = Solution::combination_sum3(3, 9);
    res.iter().for_each(|v| println!("{:?}", v));
}

struct Solution {}
