/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 * [93] 复原IP地址
 *
 * https://leetcode-cn.com/problems/restore-ip-addresses/description/
 *
 * algorithms
 * Medium (42.77%)
 * Total Accepted:    6.3K
 * Total Submissions: 14.7K
 * Testcase Example:  '"25525511135"'
 *
 * 给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。
 *
 * 示例:
 *
 * 输入: "25525511135"
 * 输出: ["255.255.11.135", "255.255.111.35"]
 *
 */
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs<'a>(res: &mut Vec<String>, s: &'a String, idx: usize, temp: &mut Vec<&'a str>) {
            if temp.len() == 4 {
                if idx == s.len() {
                    res.push(temp.join("."));
                }
                return;
            }

            for i in 1..=3 {
                if idx + i > s.len() {
                    continue;
                }
                let v = &s[idx..(idx + i)];
                if v.len() == 3 && v > "255" {
                    continue;
                }
                if v.len() != 1 && v.starts_with('0') {
                    continue;
                }

                temp.push(v);
                dfs(res, s, idx + i, temp);
                temp.pop();
            }
        }
        let mut res = vec![];

        dfs(&mut res, &s, 0, &mut vec![]);
        res
    }
}

fn main() {
    fn check(s: &str) {
        let res = Solution::restore_ip_addresses(s.to_string());
        dbg!(res);
    }
    check("010010");
    check("192168102");
}

struct Solution {}
