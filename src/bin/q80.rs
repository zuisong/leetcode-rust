/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除排序数组中的重复项 II
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array-ii/description/
 *
 * algorithms
 * Medium (48.20%)
 * Total Accepted:    10K
 * Total Submissions: 20.7K
 * Testcase Example:  '[1,1,1,2,2,3]'
 *
 * 给定一个排序数组，你需要在原地删除重复出现的元素，使得每个元素最多出现两次，返回移除后数组的新长度。
 *
 * 不要使用额外的数组空间，你必须在原地修改输入数组并在使用 O(1) 额外空间的条件下完成。
 *
 * 示例 1:
 *
 * 给定 nums = [1,1,1,2,2,3],
 *
 * 函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3 。
 *
 * 你不需要考虑数组中超出新长度后面的元素。
 *
 * 示例 2:
 *
 * 给定 nums = [0,0,1,1,1,1,2,3,3],
 *
 * 函数应返回新长度 length = 7, 并且原数组的前五个元素被修改为 0, 0, 1, 1, 2, 3, 3 。
 *
 * 你不需要考虑数组中超出新长度后面的元素。
 *
 *
 * 说明:
 *
 * 为什么返回数值是整数，但输出的答案是数组呢?
 *
 * 请注意，输入数组是以“引用”方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
 *
 * 你可以想象内部操作如下:
 *
 * // nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
 * int len = removeDuplicates(nums);
 *
 * // 在函数里修改输入数组对于调用者是可见的。
 * // 根据你的函数返回的长度, 它会打印出数组中该长度范围内的所有元素。
 * for (int i = 0; i < len; i++) {
 * print(nums[i]);
 * }
 *
 */
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut i = 1;

        for j in 2..nums.len() {
            if nums[j] != nums[i - 1] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        (i + 1) as i32
    }
}

fn main() {
    let mut v = vec![1, 1, 2, 2, 2, 3, 3, 4, 5, 6, 6, 7];
    let s = Solution::remove_duplicates(&mut v);
    let x: Vec<_> = v.iter().take(s as usize).collect();
    dbg!(x);
    dbg!(s);
}

struct Solution {}
