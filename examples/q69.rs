/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根
 *
 * https://leetcode-cn.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (34.96%)
 * Total Accepted:    28.2K
 * Total Submissions: 80.2K
 * Testcase Example:  '4'
 *
 * 实现 int sqrt(int x) 函数。
 *
 * 计算并返回 x 的平方根，其中 x 是非负整数。
 *
 * 由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。
 *
 * 示例 1:
 *
 * 输入: 4
 * 输出: 2
 *
 *
 * 示例 2:
 *
 * 输入: 8
 * 输出: 2
 * 说明: 8 的平方根是 2.82842...,
 * 由于返回类型是整数，小数部分将被舍去。
 *
 *
 */
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            unreachable!();
        }
        if x <= 1 {
            return x;
        }
        let mut i = 0;
        let mut j = x;

        while i + 1 < j {
            let mid = (i + j) / 2;

            if mid == x / mid {
                return mid;
            }

            if mid > x / mid {
                j = mid;
            } else {
                i = mid;
            }
        }
        i
    }
}

fn main() {
    vec![80, 82, 1, 0, 5, 10, 2147395599].iter().for_each(|it| {
        assert_eq!(f64::sqrt(*it as f64) as i32, Solution::my_sqrt(*it));
    });
}

struct Solution {}
