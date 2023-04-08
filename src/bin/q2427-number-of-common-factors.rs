/*
给你两个正整数 a 和 b ，返回 a 和 b 的 公 因子的数目。

如果 x 可以同时整除 a 和 b ，则认为 x 是 a 和 b 的一个 公因子 。



示例 1：

输入：a = 12, b = 6
输出：4
解释：12 和 6 的公因子是 1、2、3、6 。
示例 2：

输入：a = 25, b = 30
输出：2
解释：25 和 30 的公因子是 1、5 。


提示：

1 <= a, b <= 1000


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/number-of-common-factors
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

struct Solution {}

/**
 * 首先，需要求出 a 和 b 的最大公约数，可以使用辗转相除法或更高效的欧几里得算法。然后，可以遍历最大公约数的所有因子，统计它们的数量即可。
 */
impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut a1 = a;
        let mut b1 = b;

        let mut max_factor = -1;
        loop {
            if a1 < b1 {
                let _temp = a1;
                a1 = b1;
                b1 = _temp;
            }
            let c = a1 % b1;
            if c == 0 {
                max_factor = b1;
                break;
            } else {
                a1 = c;
            }
        }
        (1..=max_factor)
            .into_iter()
            .filter(|it| max_factor % it == 0)
            .count() as i32
    }
}

fn main() {
    let res = Solution::common_factors(25, 30);
    println!("{}", res);
    // 25 和 30 的公因子是 1、5
    assert_eq!(2, res);
}
