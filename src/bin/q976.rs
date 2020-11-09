use std::cmp::max;

use serde_json::ser::CharEscape::Solidus;

/**
给定由一些正数（代表长度）组成的数组 A，返回由其中三个长度组成的、面积不为零的三角形的最大周长。

如果不能形成任何面积不为零的三角形，返回0。



示例 1：

输入：[2,1,2]
输出：5
示例 2：

输入：[1,2,1]
输出：0
示例 3：

输入：[3,2,3,4]
输出：10
示例 4：

输入：[3,6,2,3]
输出：8


提示：

3 <= A.length <= 10000
1 <= A[i] <= 10^6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/largest-perimeter-triangle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut aa = [a[0], a[1], a[2]];


        for i in 2..a.len() {
            if a[i] > aa[0] {
                aa[0] = a[i];
                aa = Solution::sort3(aa);
                if aa[1] + aa[0] > aa[2] && aa[1] + aa[2] + aa[0] > res {
                    res = aa[1] + aa[2] + aa[0];
                }
            }
        }

        res
    }

    fn sort2(a: (i32, i32)) -> (i32, i32) {
        if a.0 > a.1 {
            (a.1, a.0)
        } else {
            a
        }
    }

    fn sort3(a: [i32; 3]) -> [i32; 3] {
        let (m1, m2) = Solution::sort2((a[0], a[1]));

        let (m3, m4) = Solution::sort2((m2, a[2]));

        let (m1, m2) = Solution::sort2((m3, m1));

        return [m1, m2, m4];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let i
            = Solution::largest_perimeter(vec![3, 6, 2, 3]);

        assert_eq!(i, 8)
    }
}

fn main() {}
