/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 *
 * https://leetcode-cn.com/problems/combination-sum-ii/description/
 *
 * algorithms
 * Medium (53.46%)
 * Total Accepted:    8.6K
 * Total Submissions: 16.2K
 * Testcase Example:  '[10,1,2,7,6,1,5]\n8'
 *
 * 给定一个数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 *
 * candidates 中的每个数字在每个组合中只能使用一次。
 *
 * 说明：
 *
 *
 * 所有数字（包括目标数）都是正整数。
 * 解集不能包含重复的组合。 
 *
 *
 * 示例 1:
 *
 * 输入: candidates = [10,1,2,7,6,1,5], target = 8,
 * 所求解集为:
 * [
 * ⁠ [1, 7],
 * ⁠ [1, 2, 5],
 * ⁠ [2, 6],
 * ⁠ [1, 1, 6]
 * ]
 *
 *
 * 示例 2:
 *
 * 输入: candidates = [2,5,2,1,2], target = 5,
 * 所求解集为:
 * [
 * [1,2,2],
 * [5]
 * ]
 *
 */


impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        println!("{:?}", candidates);
        let mut res = vec![];
        Self::dfs(target, 0, &mut vec![], &candidates, &mut res);

        res.sort();

        res.dedup_by(|it1, it2| {
            it1.len() == it2.len() &&
                it1.iter().enumerate().all(|(i, val)| *val == it2[i])
        });
        res
    }

    fn dfs(target: i32, idx: usize, temp: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if target < 0 { return; }

        if target == 0 {
            res.push(temp.clone());
            return;
        }

        for i in idx..nums.len() {
            temp.push(nums[i]);
            Self::dfs(target - nums[i], i + 1, temp, nums, res);
            temp.pop();
        }
    }
}

fn main() {
    let res = Solution::combination_sum2(vec![2, 2, 2, 4, 3, 6, 5, 6], 8);

    println!("{:?}", res);
}

struct Solution {}

