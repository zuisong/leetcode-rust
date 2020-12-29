//给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
//
//
//
// 示例 1：
//
//
//
//
//输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
//输出：6
//解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
//
//
// 示例 2：
//
//
//输入：height = [4,2,0,3,2,5]
//输出：9
//
//
//
//
// 提示：
//
//
// n == height.length
// 0 <= n <= 3 * 104
// 0 <= height[i] <= 105
//
// Related Topics 栈 数组 双指针
// 👍 1872 👎 0

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let (max_idx, _) = height.iter().enumerate().max_by_key(|it2| it2.1).unwrap();

        let mut result = 0;

        let mut temp = *height.first().unwrap();
        for i in 1..max_idx {
            if temp > height[i] {
                result += temp - height[i];
            } else {
                temp = height[i];
            }
        }

        temp = *height.last().unwrap();
        for i in (max_idx..height.len()).rev() {
            if temp > height[i] {
                result += temp - height[i];
            } else {
                temp = height[i];
            }
        }

        return result;
    }
}

fn main() {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let res = Solution::trap(heights);

    println!("{}", res);
}
