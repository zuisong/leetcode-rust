/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode-cn.com/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (21.47%)
 * Total Accepted:    11.3K
 * Total Submissions: 52.4K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给定一个字符串 (s) 和一个字符模式 (p)。实现支持 '.' 和 '*' 的正则表达式匹配。
 *
 * '.' 匹配任意单个字符。
 * '*' 匹配零个或多个前面的元素。
 *
 *
 * 匹配应该覆盖整个字符串 (s) ，而不是部分字符串。
 *
 * 说明:
 *
 *
 * s 可能为空，且只包含从 a-z 的小写字母。
 * p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
 *
 *
 * 示例 1:
 *
 * 输入:
 * s = "aa"
 * p = "a"
 * 输出: false
 * 解释: "a" 无法匹配 "aa" 整个字符串。
 *
 *
 * 示例 2:
 *
 * 输入:
 * s = "aa"
 * p = "a*"
 * 输出: true
 * 解释: '*' 代表可匹配零个或多个前面的元素, 即可以匹配 'a' 。因此, 重复 'a' 一次, 字符串可变为 "aa"。
 *
 *
 * 示例 3:
 *
 * 输入:
 * s = "ab"
 * p = ".*"
 * 输出: true
 * 解释: ".*" 表示可匹配零个或多个('*')任意字符('.')。
 *
 *
 * 示例 4:
 *
 * 输入:
 * s = "aab"
 * p = "c*a*b"
 * 输出: true
 * 解释: 'c' 可以不被重复, 'a' 可以被重复一次。因此可以匹配字符串 "aab"。
 *
 *
 * 示例 5:
 *
 * 输入:
 * s = "mississippi"
 * p = "mis*is*p*."
 * 输出: false
 *
 */

///
/// dp[i][j] 表示 字符串 s 的 前i字符能否被pattern的前j个匹配
/// dp[0][0] = true
/// dp[i][j] = dp[i-1][j-1] && match(s[i],p[j]) || dp[i-1][j]&& p[j]==*
///
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;

        for j in (2..=p.len()).step_by(2) {
            dp[0][j] = dp[0][j - 2] && p[j - 1] == b'*';
        }

        for i in 1..=s.len() {
            for j in 1..=p.len() {
                dp[i][j] = match p[j - 1] {
                    b'*' => {
                        dp[i][j - 2]  // * 可以匹配0个
                            || dp[i][j - 1] // 可以匹配一个
                            || ((dp[i - 1][j - 1] || dp[i - 1][j])
                            && (p[j - 2] == s[i - 1] || p[j - 2] == b'.'))
                        // 匹配多个
                    }
                    b'.' => dp[i - 1][j - 1],
                    _ => dp[i - 1][j - 1] && s[i - 1] == p[j - 1],
                };
            }
        }
        dp[s.len()][p.len()]
    }
}

struct Solution {}

fn main() {
    let s = "aaa".to_string();
    let p = "ab*a*c*a".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(true, res);
    let s = "mississippi".to_string();
    let p = "mis*is*p*.".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(false, res);

    let s = "aab".to_string();
    let p = "c*a*b".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(true, res);

    let s = "aaa".to_string();
    let p = "a*".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(true, res);

    let s = "aa".to_string();
    let p = "a*".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(true, res);

    let s = "aa".to_string();
    let p = "a".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(false, res);

    let s = "".to_string();
    let p = "a*".to_string();
    let res = Solution::is_match(s, p);
    assert_eq!(true, res);
}
