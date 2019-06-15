/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 *
 * https://leetcode-cn.com/problems/wildcard-matching/description/
 *
 * algorithms
 * Hard (20.96%)
 * Total Accepted:    4.1K
 * Total Submissions: 19.3K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给定一个字符串 (s) 和一个字符模式 (p) ，实现一个支持 '?' 和 '*' 的通配符匹配。
 *
 * '?' 可以匹配任何单个字符。
 * '*' 可以匹配任意字符串（包括空字符串）。
 *
 *
 * 两个字符串完全匹配才算匹配成功。
 *
 * 说明:
 *
 *
 * s 可能为空，且只包含从 a-z 的小写字母。
 * p 可能为空，且只包含从 a-z 的小写字母，以及字符 ? 和 *。
 *
 *
 * 示例 1:
 *
 * 输入:
 * s = "aa"
 * p = "a"
 * 输出: false
 * 解释: "a" 无法匹配 "aa" 整个字符串。
 *
 * 示例 2:
 *
 * 输入:
 * s = "aa"
 * p = "*"
 * 输出: true
 * 解释: '*' 可以匹配任意字符串。
 *
 *
 * 示例 3:
 *
 * 输入:
 * s = "cb"
 * p = "?a"
 * 输出: false
 * 解释: '?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。
 *
 *
 * 示例 4:
 *
 * 输入:
 * s = "adceb"
 * p = "*a*b"
 * 输出: true
 * 解释: 第一个 '*' 可以匹配空字符串, 第二个 '*' 可以匹配字符串 "dce".
 *
 *
 * 示例 5:
 *
 * 输入:
 * s = "acdcb"
 * p = "a*c?b"
 * 输入: false
 *
 */
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let ls = s.len();
        let lp = p.len();
        let mut dp = vec![vec![false; lp + 1]; ls + 1];
        dp[0][0] = true;

        for i in 0..p.len() {
            if p[i] == b'*' {
                dp[0][i + 1] = dp[0][i];
            } else {
                break;
            }
        }

        for i in 1..=ls {
            for j in 1..=lp {
                dp[i][j] = match p[j - 1] {
                    b'*' => dp[i - 1][j] || dp[i][j - 1],
                    b'?' => dp[i - 1][j - 1],
                    _ => dp[i - 1][j - 1] && p[j - 1] == s[i - 1],
                }
            }
        }

        //dp.iter().for_each(|v| println!("{:?}", v));
        dp[ls][lp]
    }
}

fn main() {
    fn check(except: bool, s: &str, p: &str) {
        assert_eq!(except, Solution::is_match(s.to_string(), p.to_string()));
    }
    check(true, "lo", "**lo");
    check(false, "acdcb", "a*c?b");
    check(true, "aa", "a*");
    check(true, "aa", "*");
    check(true, "cb", "c?");
    check(false, "aa", "a");
    check(true, "adceb", "*a*b");
}

struct Solution {}
