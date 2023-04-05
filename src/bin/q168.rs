//给定一个正整数，返回它在 Excel 表中相对应的列名称。
//
// 例如，
//
//    1 -> A
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
        if n <= 26 {
            let mut s = String::new();
            if n > 0 {
                s.push(char::from(b'A' - 1 + n as u8));
            }
            s.shrink_to_fit();
            s
        } else {
            let mut str = String::new();
            let i = n % 26;
            if i == 0 {
                str.push_str(Self::convert_to_title(n / 26 - 1).as_str());
                str.push_str(Self::convert_to_title(26).as_str());
            } else {
                str.push_str(Self::convert_to_title(n / 26).as_str());
                str.push_str(Self::convert_to_title(i).as_str());
            }
            str.shrink_to_fit();
            str
        }
    }
}

struct Solution {}

fn main() {
    let r = Solution::convert_to_title(26 * 26 * 26);
    println!("{:?}", r);
}
