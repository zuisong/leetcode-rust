/**
 * @lc app=leetcode.cn id=220 lang=rust
 *
 * [220] 存在重复元素 III
 *
 * https://leetcode-cn.com/problems/contains-duplicate-iii/description/
 *
 * algorithms
 * Medium (23.33%)
 * Total Accepted:    3.2K
 * Total Submissions: 13.7K
 * Testcase Example:  '[1,2,3,1]\n3\n0'
 *
 * 给定一个整数数组，判断数组中是否有两个不同的索引 i 和 j，使得 nums [i] 和 nums [j] 的差的绝对值最大为 t，并且 i 和 j
 * 之间的差的绝对值最大为 ķ。
 *
 * 示例 1:
 *
 * 输入: nums = [1,2,3,1], k = 3, t = 0
 * 输出: true
 *
 * 示例 2:
 *
 * 输入: nums = [1,0,1,1], k = 1, t = 2
 * 输出: true
 *
 * 示例 3:
 *
 * 输入: nums = [1,5,9,1,5,9], k = 2, t = 3
 * 输出: false
 *
 */

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut i1: usize = 0;
        loop {
            let i2 = i1 + k as usize;

            if i2 >= nums.len() {
                break;
            }
            if (nums[i1] - nums[i2]).abs() == t {
                return true;
            }
            i1 += 1;
        }
        false
    }
}

fn main() {
    let nums = vec![1, 0, 1, 1];
    let k = 1;
    let t = 2;

    let d = Solution::contains_nearby_almost_duplicate(nums, k, t);
    println!("{}", d);
}
