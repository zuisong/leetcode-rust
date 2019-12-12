/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 *
 * https://leetcode-cn.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (25.50%)
 * Total Accepted:    6.9K
 * Total Submissions: 27K
 * Testcase Example:  '"(()"'
 *
 * 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
 *
 * 示例 1:
 *
 * 输入: "(()"
 * 输出: 2
 * 解释: 最长有效括号子串为 "()"
 *
 *
 * 示例 2:
 *
 * 输入: ")()())"
 * 输出: 4
 * 解释: 最长有效括号子串为 "()()"
 *
 *
 */
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let chars: Vec<_> = s.chars().collect();
        let l = s.len();
        let mut v = vec![0; l + 1];
        let mut res = 0;
        for i1 in 1..=l {
            let mut temp = 0;
            let mut count = 0;
            v[i1 - 1] = 0;
            for i2 in i1..=l {
                if chars[i2 - 1] == '(' {
                    v[i2] = v[i2 - 1] + 1;
                } else {
                    if v[i2 - 1] > 0 {
                        v[i2] = v[i2 - 1] - 1;
                        if v[i2] == 0 {
                            temp += count + 1;
                            res = res.max(temp);
                            count = 0;
                        } else {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        res * 2
    }
}

fn main() {}

struct Solution {}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::Solution;

    #[test_case(4, ")()())"; "1")]
    #[test_case(2, "(()"; "2")]
    #[test_case(4, "(())((("; "3")]
    #[test_case(4, "(()()(("; "4")]
    #[test_case(0, ""; "5")]
    #[test_case(4, ")()())"; "6")]
    fn test_pair(n: i32, s: &str) {
        assert_eq!(n, Solution::longest_valid_parentheses(s.to_string()));
    }

    #[test]
    fn test1() {}
}
