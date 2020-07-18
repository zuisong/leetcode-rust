/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (47.86%)
 * Total Accepted:    17.2K
 * Total Submissions: 35.9K
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
 *
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 *
 *
 *
 * 示例:
 *
 * 输入："23"
 * 输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * 说明:
 * 尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
 *
 */
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let v: Vec<_> = digits.chars().collect();
        let mut res: Vec<String> = vec![];

        let mut map: HashMap<char, Vec<u8>> = HashMap::new();
        map.insert('2', vec![b'a', b'b', b'c']);
        map.insert('3', vec![b'd', b'e', b'f']);
        map.insert('4', vec![b'g', b'h', b'i']);
        map.insert('5', vec![b'j', b'k', b'l']);
        map.insert('6', vec![b'm', b'n', b'o']);
        map.insert('7', vec![b'p', b'q', b'r', b's']);
        map.insert('8', vec![b't', b'u', b'v']);
        map.insert('9', vec![b'w', b'x', b'y', b'z']);
        dfs(&mut res, &mut vec![], &v, &map);
        res
    }
}

fn dfs(
    res: &mut Vec<String>,
    temp: &mut Vec<u8>,
    digits: &Vec<char>,
    map: &HashMap<char, Vec<u8>>,
) {
    if temp.len() == digits.len() {
        let s = String::from_utf8(temp.clone()).unwrap();
        res.push(s);
        return;
    }

    for u in map.get(&digits[temp.len()]).unwrap() {
        temp.push(*u);
        dfs(res, temp, digits, map);
        temp.pop();
    }
}

fn main() {
    let res = Solution::letter_combinations("223".to_string());
    dbg!(res);
}

struct Solution {}
