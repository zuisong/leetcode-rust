/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 *
 * https://leetcode-cn.com/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (50.77%)
 * Total Accepted:    7.8K
 * Total Submissions: 15.3K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个可包含重复数字的序列，返回所有不重复的全排列。
 *
 * 示例:
 *
 * 输入: [1,1,2]
 * 输出:
 * [
 *  [1,1,2],
 *  [1,2,1],
 *  [2,1,1]
 * ]
 *
 */

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let new_nums: Vec<usize> = (0..nums.len()).collect();
        let mut res = vec![];
        Self::dfs(&mut res, &mut vec![], &new_nums);
        let mut res: Vec<Vec<_>> = res
            .iter()
            .map(|it| (*it).iter().map(|idx| nums[*idx]).collect())
            .collect();
        res.sort();
        res.dedup_by(|it1, it2| {
            it1.len() == it2.len() && it1.iter().enumerate().all(|(i, val)| *val == it2[i])
        });
        res
    }

    fn dfs(res: &mut Vec<Vec<usize>>, temp: &mut Vec<usize>, nums: &Vec<usize>) {
        if temp.len() == nums.len() {
            res.push(temp.clone());
            return;
        }

        for num in nums {
            if !temp.contains(num) {
                temp.push(*num);
                Self::dfs(res, temp, nums);
                temp.pop();
            }
        }
    }
}

fn main() {
    let res = Solution::permute_unique(vec![1, 1, 2, 2]);
    println!("{:?}", res);
}

struct Solution {}
