/*
 * @lc app=leetcode.cn id=227 lang=rust
 *
 * [227] 基本计算器 II
 *
 * https://leetcode-cn.com/problems/basic-calculator-ii/description/
 *
 * algorithms
 * Medium (30.87%)
 * Total Accepted:    2K
 * Total Submissions: 6.5K
 * Testcase Example:  '"3+2*2"'
 *
 * 实现一个基本的计算器来计算一个简单的字符串表达式的值。
 *
 * 字符串表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
 *
 * 示例 1:
 *
 * 输入: "3+2*2"
 * 输出: 7
 *
 *
 * 示例 2:
 *
 * 输入: " 3/2 "
 * 输出: 1
 *
 * 示例 3:
 *
 * 输入: " 3+5 / 2 "
 * 输出: 5
 *
 *
 * 说明：
 *
 *
 * 你可以假设所给定的表达式都是有效的。
 * 请不要使用内置的库函数 eval。
 *
 *
 */

impl Solution {
    pub fn calculate(s: String) -> i32 {
        #[derive(Debug)]
        enum Token {
            NUM(i32),
            OPT(char),
        }
        let mut v: Vec<(char, i32)> = vec![];
        let mut output = vec![];
        let mut pre_is_numeric = false;
        for c in s.chars() {
            if !c.is_whitespace() {
                if c.is_numeric() {
                    let num = c.to_digit(10).unwrap() as i32;
                    if !pre_is_numeric {
                        output.push(Token::NUM(num));
                    } else if let Token::NUM(n) = output[output.len() - 1] {
                        let l = output.len();
                        output[l - 1] = Token::NUM(n * 10 + num);
                    }
                    pre_is_numeric = true;
                } else {
                    pre_is_numeric = false;
                }
                if c == '/' || c == '*' {
                    while !v.is_empty() && v.last().unwrap().1 >= 2 {
                        output.push(Token::OPT(v.pop().unwrap().0));
                    }
                    v.push((c, 2));
                } else if c == '+' || c == '-' {
                    while !v.is_empty() && v.last().unwrap().1 >= 1 {
                        output.push(Token::OPT(v.pop().unwrap().0));
                    }
                    v.push((c, 1));
                }
            }
        }
        while !v.is_empty() {
            output.push(Token::OPT(v.pop().unwrap().0));
        }
        println!("{:?}", output);
        let mut v: Vec<i32> = vec![];
        for c in output.iter() {
            match c {
                Token::OPT(c) => {
                    let n2 = v.pop().unwrap();
                    let n1 = v.pop().unwrap();
                    let res = match c {
                        '+' => n1 + n2,
                        '-' => n1 - n2,
                        '*' => n1 * n2,
                        '/' => n1 / n2,
                        _ => unreachable!(),
                    };
                    v.push(res);
                }
                Token::NUM(n) => v.push(*n),
            }
        }
        v.pop().unwrap()
    }
}

fn main() {
    let r = Solution::calculate("1*2-3/4+5*6-7*8+9/11".to_string());
    dbg!(r);
    let r = Solution::calculate("100000000/1/2/3/4/5/6/7/8/9/10".to_string());
    dbg!(r);
}

struct Solution {}
