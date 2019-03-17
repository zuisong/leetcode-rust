/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 *
 * https://leetcode-cn.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (46.61%)
 * Total Accepted:    18.5K
 * Total Submissions: 39.3K
 * Testcase Example:  '"11"\n"1"'
 *
 * 给定两个二进制字符串，返回他们的和（用二进制表示）。
 *
 * 输入为非空字符串且只包含数字 1 和 0。
 *
 * 示例 1:
 *
 * 输入: a = "11", b = "1"
 * 输出: "100"
 *
 * 示例 2:
 *
 * 输入: a = "1010", b = "1011"
 * 输出: "10101"
 *
 */
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {

        let num1: VecDeque<u8> = a.chars().map(|c| c.to_string().parse().unwrap()).collect();
        let num2: VecDeque<u8> = b.chars().map(|c| c.to_string().parse().unwrap()).collect();

        add(&num1, &num2)
            .iter().map(|it| it.to_string()).collect()
    }
}

/// 两个大数相加  [1,2,4] + [4,8] = [1,7,2]
fn add(num1: &VecDeque<u8>, num2: &VecDeque<u8>) -> VecDeque<u8> {
    if num1.len() < num2.len() {
        return add(num2, num1);
    }
    let (l1, l2) = (num1.len() - 1, num2.len() - 1);

    let mut res = VecDeque::new();// 保存结果

    let mut flag = 0; // 进位标识
    for i in 0..num2.len() {
        let t = num1[l1 - i] + num2[l2 - i] + flag;
        res.push_front(t % 2);
        flag = t / 2;
    }
    for i in num2.len()..num1.len() {
        let t = num1[l1 - i] + flag;
        res.push_front(t % 2);
        flag = t / 2;
    }
    if flag != 0 {
        res.push_front(flag);
    }
    res
}


fn main() {
    let num1: u64 = 101101;
    let num2: u64 = 110101;
    let s = Solution::add_binary(num1.to_string(), num2.to_string());
    println!("{},{}", s, num1 + num2);
}

struct Solution {}
