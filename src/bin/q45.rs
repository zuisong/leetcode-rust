/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 *
 * https://leetcode-cn.com/problems/jump-game-ii/description/
 *
 * algorithms
 * Hard (28.97%)
 * Total Accepted:    7.1K
 * Total Submissions: 24.2K
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * 给定一个非负整数数组，你最初位于数组的第一个位置。
 *
 * 数组中的每个元素代表你在该位置可以跳跃的最大长度。
 *
 * 你的目标是使用最少的跳跃次数到达数组的最后一个位置。
 *
 * 示例:
 *
 * 输入: [2,3,1,1,4]
 * 输出: 2
 * 解释: 跳到最后一个位置的最小跳跃数是 2。
 * 从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 *
 *
 * 说明:
 *
 * 假设你总是可以到达数组的最后一个位置。
 *
 */

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        // 保存跳 jump 步 可以到达 并且 jump-1 步不能到达 的最左边点
        let mut start = 0;
        // 保存跳 jump 步 可以到达的最右边点
        let mut end = 0;
        // 当前跳了几步
        let mut jump = 0;
        while end < nums.len() - 1 {
            let temp = end;
            for j in (start..=end).rev() {
                end = end.max(j + nums[j] as usize);
            }
            // 上一次可以到达的最右边点+1作为新一次起跳的起点
            start = temp + 1;
            jump += 1;
        }
        jump
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4, 1];
    let s = Solution::jump(nums);
    dbg!(s);
}

struct Solution {}
