/**
 * @lc app=leetcode.cn id=204 lang=rust
 *
 * [204] 计数质数
 *
 * https://leetcode-cn.com/problems/count-primes/description/
 *
 * algorithms
 * Easy (26.41%)
 * Total Accepted:    12.8K
 * Total Submissions: 48.5K
 * Testcase Example:  '10'
 *
 * 统计所有小于非负整数 n 的质数的数量。
 *
 * 示例:
 *
 * 输入: 10
 * 输出: 4
 * 解释: 小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
 *
 *
 */

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut ve = vec![0; n as usize];
        ve[0] = -1;
        ve[1] = -1;
        let mut result = 0;

        for i in 2..n as usize {
            if ve[i] == 0 {
                result += 1;
                let mut num: usize = i as usize;
                while num < n as usize {
                    ve[num] = -1;
                    num += i;
                }
            }
        }
        result
    }
}

fn main() {
    let s = Solution::count_primes(100);
    println!("{:?}", s);
}

struct Solution {}
