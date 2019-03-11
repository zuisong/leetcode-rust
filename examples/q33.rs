/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 *
 * https://leetcode-cn.com/problems/search-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (35.68%)
 * Total Accepted:    16.3K
 * Total Submissions: 45.5K
 * Testcase Example:  '[4,5,6,7,0,1,2]\n0'
 *
 * 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
 *
 * ( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。
 *
 * 搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
 *
 * 你可以假设数组中不存在重复的元素。
 *
 * 你的算法时间复杂度必须是 O(log n) 级别。
 *
 * 示例 1:
 *
 * 输入: nums = [4,5,6,7,0,1,2], target = 0
 * 输出: 4
 *
 *
 * 示例 2:
 *
 * 输入: nums = [4,5,6,7,0,1,2], target = 3
 * 输出: -1
 *
 */

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn s(nums: &Vec<i32>, target: &i32, left: usize, right: usize) -> i32 {
            if *target == nums[left] {
                return left as i32;
            }

            if *target == nums[right] {
                return right as i32;
            }
            if left >= right { return -1; }

            let mid = (left + right) / 2;
            if nums[mid] == *target { return mid as i32; };
            if nums[left] < nums[mid] {
                if *target < nums[mid] && *target > nums[left] {
                    if mid == 0 { return -1; }
                    return s(nums, target, left + 1, mid - 1);
                } else {
                    return s(nums, target, mid + 1, right - 1);
                };
            } else {
                if *target > nums[mid] && *target < nums[right] {
                    return s(nums, target, mid + 1, right);
                } else {
                    if mid == 0 { return -1; }
                    return s(nums, target, left + 1, mid);
                };
            }
        }
        if nums.is_empty() {
            return -1;
        }
        return s(&nums, &target, 0, nums.len() - 1);
    }
}

fn main() {
    let s = Solution::search(vec![8, 9, 2, 3, 4], 9);
    println!("{}", s)
}

struct Solution {}
