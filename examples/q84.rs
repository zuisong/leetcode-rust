// https://leetcode-cn.com/problems/largest-rectangle-in-histogram/
// 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
//
// 求在该柱状图中，能够勾勒出来的矩形的最大面积。





pub struct Solution {}


impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        let mut result = 0;

        for height in heights {
            if stack.last().unwrap_or(&0) > &height {
                let mut i = 0;
                while stack.last().unwrap_or(&0) > &height {
                    i += 1;
                    result = result.max(stack.pop().unwrap() * i);
                };
                for _ in 0..=i { stack.push(height) }
            } else {
                stack.push(height);
            };
        };
        result = stack.iter()
            .enumerate()
            .fold(result, |a, (idx, v)| a.max(v * (stack.len() - idx) as i32));


        return result;
    }
}

fn main() {
    let res = Solution::largest_rectangle_area(vec![4, 3, 5, 6]);
    println!("{}", res);
}