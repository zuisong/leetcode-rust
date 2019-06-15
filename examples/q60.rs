/**
 * @lc app=leetcode.cn id=60 lang=rust
 *
 * [60] 第k个排列
 *
 * https://leetcode-cn.com/problems/permutation-sequence/description/
 *
 * algorithms
 * Medium (45.34%)
 * Total Accepted:    6.3K
 * Total Submissions: 13.9K
 * Testcase Example:  '3\n3'
 *
 * 给出集合 [1,2,3,…,n]，其所有元素共有 n! 种排列。
 *
 * 按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：
 *
 *
 * "123"
 * "132"
 * "213"
 * "231"
 * "312"
 * "321"
 *
 *
 * 给定 n 和 k，返回第 k 个排列。
 *
 * 说明：
 *
 *
 * 给定 n 的范围是 [1, 9]。
 * 给定 k 的范围是[1,  n!]。
 *
 *
 * 示例 1:
 *
 * 输入: n = 3, k = 3
 * 输出: "213"
 *
 *
 * 示例 2:
 *
 * 输入: n = 4, k = 9
 * 输出: "2314"
 *
 *
 */
impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        fn j(n: i32) -> i32 {
            (1..n).fold(1, |a, b| a * b)
        }
        let mut v: Vec<i32> = (1..=n).collect();
        let mut res = vec![];
        k -= 1;
        while !v.is_empty() {
            let jn = j(v.len() as i32);
            let idx = k / jn;
            let n = v.remove(idx as usize);
            res.push(n);
            k = k % jn;
        }

        for n in v {
            res.push(n)
        }

        res.iter()
            .map(|it| (*it).to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

struct Solution {}

fn main() {
    fn check(m: i32, k: i32) {
        let res = Solution::get_permutation(m, k);
        println!("{:?}", res);
    }
    check(3, 1);
    check(3, 2);
    check(3, 3);
    check(3, 4);
    check(3, 5);
    check(3, 6);
    check(9, 3);
}
