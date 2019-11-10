/*
 * @lc app=leetcode.cn id=91 lang=rust
 *
 * [91] 解码方法
 *
 * https://leetcode-cn.com/problems/decode-ways/description/
 *
 * algorithms
 * Medium (19.83%)
 * Total Accepted:    5K
 * Total Submissions: 25.4K
 * Testcase Example:  '"12"'
 *
 * 一条包含字母 A-Z 的消息通过以下方式进行了编码：
 *
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 *
 *
 * 给定一个只包含数字的非空字符串，请计算解码方法的总数。
 *
 * 示例 1:
 *
 * 输入: "12"
 * 输出: 2
 * 解释: 它可以解码为 "AB"（1 2）或者 "L"（12）。
 *
 *
 * 示例 2:
 *
 * 输入: "226"
 * 输出: 3
 * 解释: 它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。
 *
 *
 */
use std::collections::HashMap;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let c: Vec<_> = s.chars().collect();
        let cs = c.as_slice();

        fn decode_count(cs: &[char], idx: usize, map: &mut HashMap<usize, i32>) -> i32 {
            if idx == cs.len() {
                return 1;
            }
            if cs[idx] == '0' {
                return 0;
            }
            if idx == cs.len() - 1 {
                return 1;
            }
            if map.contains_key(&idx) {
                return *map.get(&idx).unwrap();
            }

            let mut count = decode_count(cs, idx + 1, map);

            if cs[idx] == '1' || (cs[idx] == '2' && cs[idx + 1] <= '6') {
                count += decode_count(cs, idx + 2, map);
            }
            map.insert(idx, count);
            count
        }
        return decode_count(cs, 0, &mut HashMap::new());
    }
}
fn main() {
    let res = Solution::num_decodings("33033".to_string());
    dbg!(res);
}

struct Solution {}
