/*
 * @lc app=leetcode.cn id=443 lang=rust
 *
 * [443] 压缩字符串
 *
 * https://leetcode-cn.com/problems/string-compression/description/
 *
 * algorithms
 * Easy (32.93%)
 * Total Accepted:    2.9K
 * Total Submissions: 8.8K
 * Testcase Example:  '['a','a','b','b','c','c','c']'
 *
 * 给定一组字符，使用原地算法将其压缩。
 *
 * 压缩后的长度必须始终小于或等于原数组长度。
 *
 * 数组的每个元素应该是长度为1 的字符（不是 int 整数类型）。
 *
 * 在完成原地修改输入数组后，返回数组的新长度。
 *
 *
 *
 * 进阶：
 * 你能否仅使用O(1) 空间解决问题？
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：
 * ['a','a','b','b','c','c','c']
 *
 * 输出：
 * 返回6，输入数组的前6个字符应该是：['a','2','b','2','c','3']
 *
 * 说明：
 * 'aa'被'a2'替代。'bb'被'b2'替代。'ccc'被'c3'替代。
 *
 *
 * 示例 2：
 *
 *
 * 输入：
 * ['a']
 *
 * 输出：
 * 返回1，输入数组的前1个字符应该是：['a']
 *
 * 说明：
 * 没有任何字符串被替代。
 *
 *
 * 示例 3：
 *
 *
 * 输入：
 * ['a','b','b','b','b','b','b','b','b','b','b','b','b']
 *
 * 输出：
 * 返回4，输入数组的前4个字符应该是：['a','b','1','2']。
 *
 * 说明：
 * 由于字符'a'不重复，所以不会被压缩。'bbbbbbbbbbbb'被“b12”替代。
 * 注意每个数字在数组中都有它自己的位置。
 *
 *
 * 注意：
 *
 *
 * 所有字符都有一个ASCII值在[35, 126]区间内。
 * 1 <= len(chars) <= 1000。
 *
 *
 */

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.is_empty() {
            return 0;
        }
        if chars.len() == 1 {
            return 1;
        }
        let mut write_idx = 0;
        let mut pre_char = chars[0];
        let mut count = 1;
        for idx in 1..chars.len() {
            if chars[idx] == pre_char {
                count += 1;
            } else {
                chars[write_idx] = pre_char;
                write_idx = Solution::write(chars, write_idx, count);
                pre_char = chars[idx];
                count = 1;
            }
        }
        chars[write_idx] = pre_char;
        write_idx = Solution::write(chars, write_idx, count);

        write_idx as i32
    }

    fn write(chars: &mut Vec<char>, write_idx: usize, count: i32) -> usize {
        let mut write_idx = write_idx;
        write_idx += 1;
        if count > 1 {
            let s = format!("{}", count);
            for c in s.chars() {
                chars[write_idx] = c;
                write_idx += 1;
            }
        }
        return write_idx;
    }
}

fn main() {
    let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
    let idx = Solution::compress(&mut chars);
    dbg!(idx);
    dbg!(&chars);
}

struct Solution {}
