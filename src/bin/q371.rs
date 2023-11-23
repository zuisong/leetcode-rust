//不使用运算符 + 和 - ，计算两整数 a 、b 之和。
//
// 示例 1:
//
// 输入: a = 1, b = 2
//输出: 3
//
//
// 示例 2:
//
// 输入: a = -2, b = 3
//输出: 1
// Related Topics 位运算

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }

        let m = a ^ b;
        let n = a & b;

        Self::get_sum(m, n << 1)
    }
}

//leetcode submit region end(Prohibit modification and deletion)
struct Solution {}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use test_case::test_case;

    #[test_case(5, 5)]
    #[test_case(100000000, 5)]
    #[test_case(200000000, 5)]
    #[test_case(322222, 5)]
    fn test_sum(a: i32, b: i32) {
        let sum = Solution::get_sum(a, b);
        println!(" {} + {} = {}", a, b, sum);
        assert_eq!(a + b, sum);
    }

    #[test]
    fn test() {}
}

fn main() {}
