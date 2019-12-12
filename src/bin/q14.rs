// 编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串 ""。
//
// 示例 1:
//
// 输入: ["flower","flow","flight"]
// 输出: "fl"
// 示例 2:
//
// 输入: ["dog","racecar","car"]
// 输出: ""
// 解释: 输入不存在公共前缀。

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        dbg!(&strs);

        let strs_vec: Vec<Vec<char>> = strs.iter().map(|it| it.chars().collect()).collect();
        match strs_vec.first() {
            Some(s) => {
                for i in 0..s.len() {
                    let b = strs_vec.iter().all(|it| it.len() > i && it[i] == s[i]);

                    if !b {
                        let s: String = strs.first().unwrap().chars().take(i).collect();
                        return s.to_string();
                    };
                }
                return strs.first().unwrap().to_string();
            }
            None => "".to_string(),
        }
    }
}

struct Solution {}

fn main() {
    let strs: Vec<String> = vec!["fl", "flow", "flight"]
        .into_iter()
        .map(|it| it.into())
        .collect();
    let res = Solution::longest_common_prefix(strs);
    println!("{}", res);
}
