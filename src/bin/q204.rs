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
 * 统计所有小于非负整数 n 的质数的数量。
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
        let n = n as usize;
        if n <= 2 {
            return 0;
        }

        let mut ve = vec![true; n];
        ve[0] = false;
        ve[1] = false;
        let mut result = 1;

        for i in (3..n).step_by(2) {
            if ve[i] {
                result += 1;
                //                dbg!(&i);
                let mut num: usize = i as usize;
                while num < n {
                    ve[num] = false;
                    num += i;
                }
            }
        }

        result
    }
}

fn main() {
    let start = std::time::Instant::now();
    let s = Solution::count_primes(100_000_000);
    let end = std::time::Instant::now();
    println!("{:?}", s);
    println!("{:?}", end - start);
}

struct Solution {}
