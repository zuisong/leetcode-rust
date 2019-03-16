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
        let mut map = HashMap::new();

        for s in strs {
            let mut s1 = s.clone();
            let x = unsafe { s1.as_bytes_mut() };

            x.sort();
            let x = String::from_utf8_lossy(x).to_string();

            if !map.contains_key(&x) {
                map.insert(x.clone(), vec![]);
            }
            map.get_mut(&x).unwrap().push(s);
        }

        dbg!(&map);

        let res: Vec<Vec<String>> = map.into_iter().map(|(_, s)| s).collect();
        res
    }
}

fn main() {
    let v: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|it| it.to_string())
        .collect();

    let res = Solution::group_anagrams(v);

    println!("{:?}", res);
}

struct Solution {}
