//给定一个正整数，返回它在 Excel 表中相对应的列名称。
//
// 例如，
//
//     1 -> A
//    2 -> B
//    3 -> C
//    ...
//    26 -> Z
//    27 -> AA
//    28 -> AB
//    ...
//
//
// 示例 1:
//
// 输入: 1
//输出: "A"
//
//
// 示例 2:
//
// 输入: 28
//输出: "AB"
//
//
// 示例 3:
//
// 输入: 701
//输出: "ZY"
//
//



impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        use core::fmt::Write;
        if n <= 26 {
            let mut s = String::new();
            if n > 0 {
                s.write_char(char::from(b'A' - 1 + n as u8)).unwrap();
            }
            s.shrink_to_fit();
            return s;
        } else {
            let mut s = String::new();
            let i = n % 26;
            if i == 0 {
                s.write_str(Self::convert_to_title(n / 26 - 1).as_ref()).unwrap();
                s.write_str(Self::convert_to_title(26).as_ref()).unwrap();
            } else {
                s.write_str(Self::convert_to_title(n / 26).as_ref()).unwrap();
                s.write_str(Self::convert_to_title(i).as_ref()).unwrap();
            }
            return s;
        }
    }
}

struct Solution {}

fn main() {
    let r = Solution::convert_to_title(26 * 26 * 26);
    println!("{:?}", r);
}