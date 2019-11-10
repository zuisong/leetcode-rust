/*
给定一个包含 n + 1 个整数的数组 nums，其数字都在 1 到 n 之间（包括 1 和 n），可知至少存在一个重复的整数。假设只有一个重复的整数，找出这个重复的数。

示例 1:

输入: [1,3,4,2,2]
输出: 2
示例 2:

输入: [3,1,3,4,2]
输出: 3
说明：

不能更改原数组（假设数组是只读的）。
只能使用额外的 O(1) 的空间。
时间复杂度小于 O(n^2) 。
数组中只有一个重复的数字，但它可能不止重复出现一次。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-the-duplicate-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
impl Solution {
    ///
    /// 解析
    /// [1,3,2,1]
    /// 从 0 下标开始   0 存放的是 数1
    /// 接着看 下标1  存的是 3
    /// 接着看 下标3  存的是 1
    /// 接着看 下标1  存的是 3
    ///  ...
    /// 成了一个环   题目转换成了找到环形链表的入口
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p1: i32 = 0; //P1
        let mut p2 = nums[p1 as usize]; //P2

        while nums[p1 as usize] != nums[p2 as usize] {
            p1 = nums[p1 as usize]; // P1 每次往前走一步
            p2 = nums[nums[p2 as usize] as usize]; // P2 每次往前走 2 步
        }
        dbg!(p1);
        p1 = nums[p1 as usize];

        let mut p3 = 0;
        while nums[p1 as usize] != nums[p3 as usize] {
            p1 = nums[p1 as usize];
            p3 = nums[p3 as usize];
        }
        nums[p3 as usize]
    }
}

fn main() {
    debug_assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
    debug_assert_eq!(2, Solution::find_duplicate(vec![2, 1, 3, 4, 2]));
}

struct Solution {}
