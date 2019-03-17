/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 *
 * https://leetcode-cn.com/problems/add-strings/description/
 *
 * algorithms
 * Easy (43.32%)
 * Total Accepted:    5.3K
 * Total Submissions: 12.2K
 * Testcase Example:  '"0"\n"0"'
 *
 * 给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和。
 *
 * 注意：
 *
 *
 * num1 和num2 的长度都小于 5100.
 * num1 和num2 都只包含数字 0-9.
 * num1 和num2 都不包含任何前导零。
 * 你不能使用任何內建 BigInteger 库， 也不能直接将输入的字符串转换为整数形式。
 *
 *
 */
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1: VecDeque<u8> = num1.chars().map(|c| c.to_string().parse().unwrap()).collect();
        let num2: VecDeque<u8> = num2.chars().map(|c| c.to_string().parse().unwrap()).collect();

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
        res.push_front(t % 10);
        flag = t / 10;
    }
    for i in num2.len()..num1.len() {
        let t = num1[l1 - i] + flag;
        res.push_front(t % 10);
        flag = t / 10;
    }
    if flag != 0 {
        res.push_front(flag);
    }
    res
}


fn main() {
    let num1: u64 = 1122222;
    let num2: u64 = 0;
    let s = Solution::add_strings(num1.to_string(), num2.to_string());
    println!("{},{}", s, num1 + num2);
}

struct Solution {}

