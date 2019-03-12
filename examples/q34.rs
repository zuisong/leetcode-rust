/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 *
 * https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
 *
 * algorithms
 * Medium (35.23%)
 * Total Accepted:    12.5K
 * Total Submissions: 35.6K
 * Testcase Example:  '[5,7,7,8,8,10]\n8'
 *
 * 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
 *
 * 你的算法时间复杂度必须是 O(log n) 级别。
 *
 * 如果数组中不存在目标值，返回 [-1, -1]。
 *
 * 示例 1:
 *
 * 输入: nums = [5,7,7,8,8,10], target = 8
 * 输出: [3,4]
 *
 * 示例 2:
 *
 * 输入: nums = [5,7,7,8,8,10], target = 6
 * 输出: [-1,-1]
 *
 */

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            vec![-1, -1]
        } else {
            search(&nums, target, 0, nums.len() - 1)
        }
    }
}

fn search(nums: &Vec<i32>, target: i32, s_idx: usize, e_idx: usize) -> Vec<i32> {
    if s_idx > e_idx {
        return vec![-1, -1];
    }
    let mid = (s_idx + e_idx) / 2;
    if target == nums[mid] {
        return vec![
            search_left(nums, target, s_idx, mid),
            search_right(nums, target, mid, e_idx),
        ];
    }

    if target < nums[mid] {
        if mid == 0 {
            return vec![-1, -1];
        }
        return search(nums, target, s_idx, mid - 1);
    } else {
        return search(nums, target, mid + 1, e_idx);
    }
}

fn search_left(nums: &Vec<i32>, target: i32, s_idx: usize, e_idx: usize) -> i32 {
    if nums[s_idx] == target {
        return s_idx as i32;
    }

    let mid = (s_idx + e_idx) / 2;
    if nums[mid] == target {
        search_left(nums, target, s_idx, mid)
    } else {
        search_left(nums, target, mid + 1, e_idx)
    }
}

fn search_right(nums: &Vec<i32>, target: i32, s_idx: usize, e_idx: usize) -> i32 {
    if nums[e_idx] == target {
        return e_idx as i32;
    }

    let mid = (s_idx + e_idx) / 2 + 1;
    if nums[mid] == target {
        search_right(nums, target, mid, e_idx)
    } else {
        search_right(nums, target, s_idx, mid - 1)
    }
}

fn main() {
    let s = Solution::search_range(vec![7, 7, 7, 7, 7, 7], 6);

    println!("{:?}", s)
}

struct Solution {}