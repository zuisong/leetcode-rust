// https://leetcode-cn.com/problems/container-with-most-water/
// 给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，
// 垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器
// 可以容纳最多的水。
//
// 说明：你不能倾斜容器，且 n 的值至少为 2。
//
//
// 图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
impl Solution {
    pub fn max_(idx1: usize, idx2: usize, height: &Vec<i32>, max_area: i32) -> i32 {
        if idx2 <= idx1 {
            return max_area;
        }
        let h1 = height[idx1];
        let h2 = height[idx2];
        let cur_area = h1.min(h2) * ((idx2 - idx1) as i32);
        if h1 > h2 {
            return Self::max_(idx1, idx2 - 1, height, cur_area.max(max_area));
        } else {
            return Self::max_(idx1 + 1, idx2, height, cur_area.max(max_area));
        }
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        Self::max_(0, height.len() - 1, &height, 0)
    }
}

struct Solution {}

fn main() {
    let res = Solution::max_area(vec![1, 2, 3, 4, 5, 6]);
    println!("{}", res);
}
