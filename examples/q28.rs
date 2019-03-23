/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 实现strStr()
 *
 * https://leetcode-cn.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (37.94%)
 * Total Accepted:    42.4K
 * Total Submissions: 111.4K
 * Testcase Example:  '"hello"\n"ll"'
 *
 * 实现 strStr() 函数。
 *
 * 给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置
 * (从0开始)。如果不存在，则返回  -1。
 *
 * 示例 1:
 *
 * 输入: haystack = "hello", needle = "ll"
 * 输出: 2
 *
 *
 * 示例 2:
 *
 * 输入: haystack = "aaaaa", needle = "bba"
 * 输出: -1
 *
 *
 * 说明:
 *
 * 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
 *
 * 对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。
 *
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
//        haystack.as_str()
//            .find(needle.as_str())
//            .map(|it| it as i32).unwrap_or(-1)
        if needle.is_empty() { return 0; }
        let p: Vec<_> = needle.chars().collect();
        let s: Vec<_> = haystack.chars().collect();

        let next = Self::get_next_vec(&p);

        let mut i = 0;
        let mut j: i32 = 0;

        while i < s.len() {
            if j == -1 || s[i] == p[j as usize] {
                i += 1;
                j += 1;
                if j == p.len() as i32 {
                    return (i - p.len()) as i32;
                }
            } else {
                j = next[j as usize];
            }
        }


        -1
    }
    ///
    /// 得到 KMP 算法中的 NEXT 数组
    ///
    fn get_next_vec(word: &Vec<char>) -> Vec<i32> {
        let mut next: Vec<i32> = vec![0; word.len()];
        next[0] = 0;
        let mut i = 1;
        let mut j = 0;//
        while i < word.len() {
            if word[i] == word[j] {
                next[i] = (j + 1) as i32;
                j += 1;
                i += 1;
            } else {
                if j == 0 {
                    next[i] = 0;
                    i += 1;
                } else {
                    j = next[j - 1] as usize;
                }
            }
        }


        for i in (1..next.len()).rev() {
            next[i] = next[i - 1];
        }
        next[0] = -1;

        return next;
    }
}


fn main() {
    fn check(s: &str, p: &str) -> i32 {
        let res = Solution::str_str(s.to_string(), p.to_string());

        assert_eq!(res, s.find(p).map(|i| i as i32).unwrap_or(-1));

        res
    }
    assert_eq!(2, check("hello", "ll"));
    assert_eq!(4, check("aaaaaab", "aab"));
    assert_eq!(-1, check("hello", "lla"));
    assert_eq!(4, check("aabaaabaaac", "aabaaac"));
    assert_eq!(6, check("ababcaababcaabc", "ababcaabc"));
}

struct Solution {}