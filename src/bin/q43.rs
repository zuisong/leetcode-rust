/*
 * @lc app=leetcode.cn id=43 lang=rust
 *
 * [43] 字符串相乘
 *
 * https://leetcode-cn.com/problems/multiply-strings/description/
 *
 * algorithms
 * Medium (37.80%)
 * Total Accepted:    11.3K
 * Total Submissions: 29.9K
 * Testcase Example:  '"2"\n"3"'
 *
 * 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
 *
 * 示例 1:
 *
 * 输入: num1 = "2", num2 = "3"
 * 输出: "6"
 *
 * 示例 2:
 *
 * 输入: num1 = "123", num2 = "456"
 * 输出: "56088"
 *
 * 说明：
 *
 *
 * num1 和 num2 的长度小于110。
 * num1 和 num2 只包含数字 0-9。
 * num1 和 num2 均不以零开头，除非是数字 0 本身。
 * 不能使用任何标准库的大数类型（比如 BigInteger）或直接将输入转换为整数来处理。
 *
 *
 */
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: VecDeque<u8> = num1
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let num2: VecDeque<u8> = num2
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        multiply(num1, num2)
            .iter()
            .map(|it| it.to_string())
            .collect()
    }
}

/// 两个大数相乘
fn multiply(num1: VecDeque<u8>, num2: VecDeque<u8>) -> VecDeque<u8> {
    let mut res = VecDeque::new();
    res.push_front(0);
    for (i, num) in num2.iter().rev().enumerate() {
        let mut t = n_multiply_one(&num1, num);
        for _ in 0..i {
            t.push_back(0);
        }
        res = add(&res, &t)
    }

    while !res.is_empty() && res[0] == 0 {
        res.pop_front();
    }
    if res.is_empty() {
        res.push_front(0);
    }
    res
}

/// 一个大数乘以一位数    [1,2,4] * 4 = [4,9,6]
fn n_multiply_one(nums: &VecDeque<u8>, n: &u8) -> VecDeque<u8> {
    let mut flag = 0; // 进位标识
    let mut res = VecDeque::new();
    for num in nums.iter().rev() {
        let t = (num * n) + flag;
        res.push_front(t % 10);
        flag = t / 10;
    }
    if flag != 0 {
        res.push_front(flag);
    }
    res
}

/// 两个大数相加  [1,2,4] + [4,8] = [1,7,2]
fn add(num1: &VecDeque<u8>, num2: &VecDeque<u8>) -> VecDeque<u8> {
    if num1.len() < num2.len() {
        return add(num2, num1);
    }
    let (l1, l2) = (num1.len() - 1, num2.len() - 1);

    let mut res = VecDeque::new(); // 保存结果

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
    let s = Solution::multiply(num1.to_string(), num2.to_string());
    println!("{},{}", s, num1 * num2);
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[quickcheck]
    fn quickcheck_test(num1: u64, num2: u64) {
        let result = Solution::multiply(num1.to_string(), num2.to_string());
        println!("{} x {} = {:?}, {:?}", num1, num2, num1 * num2, result);

        assert_eq!((num1 * num2).to_string(), result)
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let num1: u64 = 1122222;
        let num2: u64 = 454654;
        assert_eq!(
            ("0").to_string(),
            Solution::multiply(num1.to_string(), num2.to_string())
        )
    }
}

struct Solution {}
