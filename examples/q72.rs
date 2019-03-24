/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 *
 * https://leetcode-cn.com/problems/edit-distance/description/
 *
 * algorithms
 * Hard (47.85%)
 * Total Accepted:    4.5K
 * Total Submissions: 9.4K
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给定两个单词 word1 和 word2，计算出将 word1 转换成 word2 所使用的最少操作数 。
 *
 * 你可以对一个单词进行如下三种操作：
 *
 *
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 *
 *
 * 示例 1:
 *
 * 输入: word1 = "horse", word2 = "ros"
 * 输出: 3
 * 解释:
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 *
 *
 * 示例 2:
 *
 * 输入: word1 = "intention", word2 = "execution"
 * 输出: 5
 * 解释:
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 *
 *
 */

use std::collections::HashMap;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<_> = word1.chars().collect();
        let mut word2: Vec<_> = word2.chars().collect();
        let l1 = word1.len();
        let l2 = word2.len();
        let mut dp = vec![vec![0; l1 + 1]; l2 + 1];

        let mut v = HashMap::new();

        for i in 1..=l1 {
            v.insert((0, i), (0, i - 1));
            dp[0][i] = i;
        }
        for i in 1..=l2 {
            v.insert((i, 0), (i - 1, 0));
            dp[i][0] = i;
        }

        for i in 1..=l2 {
            for j in 1..=l1 {
                if word2[i - 1] == word1[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    v.insert((i, j), (i - 1, j - 1));
                } else {
                    let m = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]);
                    dp[i][j] = m + 1;

                    if m == dp[i - 1][j - 1] {
                        v.insert((i, j), (i - 1, j - 1))
                    } else if m == dp[i][j - 1] {
                        v.insert((i, j), (i, j - 1))
                    } else {
                        v.insert((i, j), (i - 1, j))
                    };
                }
            }
        }

        dp.iter().for_each(|it| println!("{:?}", it));
        println!();
        let mut route = Vec::new();
        let mut temp = (l2, l1);
        while temp.0 != 0 || temp.1 != 0 {
            route.push(temp);
            temp = *v.get(&temp).unwrap();
        }
        route.push(temp);
        route.iter().for_each(|it| println!("{:?}", it));


        let mut route_str: Vec<String> = vec![];
        route_str.push(word2.iter().collect());

        for i in 0..route.len() - 1 {
            let p1 = route[i + 1];
            let p2 = route[i];

            if dp[p1.0][p1.1] == dp[p2.0][p2.1] {
                continue;
            }

            if p1.0 + 1 == p2.0 && p1.1 + 1 == p2.1 {
                word2[p2.0 - 1] = word1[p2.1 - 1];
            };
            if p1.0 + 1 == p2.0 && p1.1 == p2.1 {
                word2.remove(p2.0 - 1);
            };

            if p1.0 == p2.0 && p1.1 + 1 == p2.1 {
                word2.insert(p2.0, word1[p1.1]);
            };
            route_str.push(word2.iter().collect());
        };
        route_str.iter().for_each(|it| println!("{:?}", it));
        dp[l2][l1] as i32
    }
}

fn main() {
    fn check(s1: &str, s2: &str) -> i32 {
        Solution::min_distance(s1.to_string(), s2.to_string())
    }
    check("intention", "execution");
    check("hello", "word");
    check("chenjian", "jiankang");
    check("天气很好", "好天气");
    check("天气很好", "晴天，很好");
}

struct Solution {}
