/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 *
 * https://leetcode-cn.com/problems/combination-sum/description/
 *
 * algorithms
 * Medium (62.38%)
 * Total Accepted:    12.1K
 * Total Submissions: 19.4K
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * 给定一个无重复元素的数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 *
 * candidates 中的数字可以无限制重复被选取。
 *
 * 说明：
 *
 *
 * 所有数字（包括 target）都是正整数。
 * 解集不能包含重复的组合。 
 *
 *
 * 示例 1:
 *
 * 输入: candidates = [2,3,6,7], target = 7,
 * 所求解集为:
 * [
 * ⁠ [7],
 * ⁠ [2,2,3]
 * ]
 *
 *
 * 示例 2:
 *
 * 输入: candidates = [2,3,5], target = 8,
 * 所求解集为:
 * [
 * [2,2,2,2],
 * [2,3,3],
 * [3,5]
 * ]
 *
 */
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::dfs(target, 0, &mut vec![], &candidates, &mut res);
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
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            Self::dfs(target - nums[i], i, temp, nums, res);
            temp.pop();
        }
    }
}

fn main() {
    let res = Solution::combination_sum(vec![2, 3, 4, 5, 6], 8);

    println!("{:?}", res);


}

struct Solution {}