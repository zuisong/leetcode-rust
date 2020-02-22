/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 *
 * https://leetcode-cn.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (54.26%)
 * Total Accepted:    11.8K
 * Total Submissions: 21.7K
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * 给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。
 *
 * 示例:
 *
 * 输入: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * 输出:
 * [
 * ⁠ ["ate","eat","tea"],
 * ⁠ ["nat","tan"],
 * ⁠ ["bat"]
 * ]
 *
 * 说明：
 *
 *
 * 所有输入均为小写字母。
 * 不考虑答案输出的顺序。
 *
 *
 */
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::new(), |mut map, s| {
                let hash = s.as_bytes().iter().fold([0; 26], |mut hash, &c| {
                    hash[(c - b'a') as usize] += 1u8;
                    hash
                });

                map.entry(hash).or_insert(vec![]).push(s);
                map
            })
            .into_iter()
            .map(|e| e.1)
            .collect()
    }
}

fn main() {
    let v: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|it| it.to_string())
        .collect();

    let res = Solution::group_anagrams(v);
    res.iter().for_each(|it| {
        println!("{:?}", it);
    })
}

struct Solution {}
